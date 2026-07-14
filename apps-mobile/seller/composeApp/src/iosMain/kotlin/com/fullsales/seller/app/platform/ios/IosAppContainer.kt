package com.fullsales.seller.app.platform.ios

import com.fullsales.seller.app.platform.IosPathNetworkMonitor
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.platform.SellerTokenStore
import com.fullsales.seller.shared.api.AuthTokenProvider
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.SellerSyncTransport
import com.fullsales.seller.shared.api.TokenRefreshHandler
import com.fullsales.seller.shared.api.apiBaseUrl
import com.fullsales.seller.shared.api.createSellerHttpClient
import com.fullsales.seller.shared.auth.SellerRoleGateResult
import com.fullsales.seller.shared.auth.gateSellerAccessToken
import com.fullsales.seller.shared.connectivity.OnlineSyncTrigger
import com.fullsales.seller.shared.db.sqldelight.SqlDelightCatalogRepository
import com.fullsales.seller.shared.db.sqldelight.SqlDelightCommerceAddressCache
import com.fullsales.seller.shared.db.sqldelight.SqlDelightMediaUrlCacheStore
import com.fullsales.seller.shared.db.sqldelight.SqlDelightOutboxRepository
import com.fullsales.seller.shared.db.sqldelight.SqlDelightRegistrationRepository
import com.fullsales.seller.shared.db.sqldelight.SqlDelightSaleRepository
import com.fullsales.seller.shared.db.sqldelight.SqlDelightSiteSettingsRepository
import com.fullsales.seller.shared.db.sqldelight.SqlDelightStockSnapshotRepository
import com.fullsales.seller.shared.db.sqldelight.createIosSellerSqlDriver
import com.fullsales.seller.shared.db.sqldelight.createSellerLocalDatabase
import com.fullsales.seller.shared.media.MediaUrlCacheEntry
import com.fullsales.seller.shared.media.MediaUrlCacheResolver
import com.fullsales.seller.shared.media.MediaUrlCacheStore
import com.fullsales.seller.shared.media.parseMediaExpiresAtEpochMs
import com.fullsales.seller.shared.media.productThumbnailLoadUrl
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.CommerceAddressCache
import com.fullsales.seller.shared.repository.RegistrationRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SiteSettingsRepository
import com.fullsales.seller.shared.repository.StockSnapshotRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import com.fullsales.seller.shared.sync.CatalogPullSync
import com.fullsales.seller.shared.sync.IosForegroundSync
import com.fullsales.seller.shared.sync.OfflineRegistrationWriter
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import com.fullsales.seller.shared.sync.PullRegistrationsSync
import com.fullsales.seller.shared.sync.PullSalesSync
import com.fullsales.seller.shared.sync.PullSettingsSync
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.sync.SyncEngine
import com.fullsales.seller.shared.sync.SyncTokenRefresher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob

/**
 * iOS DI — LocalStore is durable SQLDelight SQLite (OD-16-5=A / Phase 16G).
 * Background WorkManager equivalent is not available; drain on Online + [onAppResume].
 */
class IosAppContainer : SellerAppContainer {
    private val sqlDriver = createIosSellerSqlDriver()
    private val localDb = createSellerLocalDatabase(sqlDriver)

    override val tokenStore: SellerTokenStore = KeychainSellerTokenStore()
    override val catalogRepository: CatalogRepository = SqlDelightCatalogRepository(localDb)
    override val saleRepository: SaleRepository = SqlDelightSaleRepository(localDb)
    override val outboxRepository: SyncOutboxRepository = SqlDelightOutboxRepository(localDb)
    override val stockSnapshots: StockSnapshotRepository = SqlDelightStockSnapshotRepository(localDb)
    override val commerceAddressCache: CommerceAddressCache = SqlDelightCommerceAddressCache(localDb)
    override val siteSettingsRepository: SiteSettingsRepository = SqlDelightSiteSettingsRepository(localDb)
    private val mediaStore: MediaUrlCacheStore = SqlDelightMediaUrlCacheStore(localDb)
    private val tokenProvider = AuthTokenProvider { tokenStore.getAccessToken() }
    private val authApiClient = SellerApiClient(createSellerHttpClient(AuthTokenProvider { null }))
    private val tokenRefresher = IosTokenRefresher(tokenStore, authApiClient)
    private val httpClient = createSellerHttpClient(tokenProvider, tokenRefresher)
    override val apiClient = SellerApiClient(httpClient)
    override val mediaUrlResolver: MediaUrlResolver = IosMediaUrlResolver(apiClient, mediaStore)
    private val syncTransport = SellerSyncTransport(apiClient)
    override val offlineSaleWriter = OfflineSaleWriter(saleRepository, outboxRepository)
    override val registrationRepository: RegistrationRepository =
        SqlDelightRegistrationRepository(localDb)
    override val offlineRegistrationWriter =
        OfflineRegistrationWriter(registrationRepository, outboxRepository)
    override val syncCoordinator = SellerSyncCoordinator(
        CatalogPullSync(catalogRepository, syncTransport),
        PullSalesSync(saleRepository, syncTransport),
        PullRegistrationsSync(registrationRepository, syncTransport),
        PullSettingsSync(siteSettingsRepository, syncTransport),
        SyncEngine(
            outboxRepository,
            saleRepository,
            syncTransport,
            tokenRefresher,
            registrationRepository,
        ),
    )
    override val networkMonitor: NetworkMonitor = IosPathNetworkMonitor()
    private val appScope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val foregroundSync = IosForegroundSync(syncCoordinator)

    init {
        OnlineSyncTrigger(
            networkMonitor.connectivity,
            syncCoordinator::pushOutbox,
            appScope,
        )
    }

    override fun requestSync() {
        // Documented limit: no WorkManager equivalent; use OnlineSyncTrigger + onAppResume.
    }

    suspend fun onAppResume() {
        foregroundSync.onAppResume()
    }
}

private class IosTokenRefresher(
    private val tokenStore: SellerTokenStore,
    private val authApiClient: SellerApiClient,
) : SyncTokenRefresher, TokenRefreshHandler {
    override suspend fun refreshToken(): Boolean = refreshTokens()

    override suspend fun refreshTokens(): Boolean {
        val refresh = tokenStore.getRefreshToken() ?: return false
        return runCatching {
            val response = authApiClient.refresh(refresh)
            when (gateSellerAccessToken(response.accessToken)) {
                is SellerRoleGateResult.Accepted -> {
                    tokenStore.saveTokens(response.accessToken, response.refreshToken)
                    true
                }
                else -> {
                    tokenStore.clear()
                    false
                }
            }
        }.getOrDefault(false)
    }
}

private class IosMediaUrlResolver(
    private val apiClient: SellerApiClient,
    store: MediaUrlCacheStore,
) : MediaUrlResolver {
    private val resolver = MediaUrlCacheResolver(
        store = store,
        fetch = { fileId ->
            val response = runCatching { apiClient.getMediaUrl(fileId) }.getOrNull()
                ?: return@MediaUrlCacheResolver null
            val loadable = productThumbnailLoadUrl(response.url, apiBaseUrl)
            val expiresAt = parseMediaExpiresAtEpochMs(response.expiresAt)
                ?: return@MediaUrlCacheResolver null
            MediaUrlCacheEntry(fileId, loadable, expiresAt)
        },
    )

    override suspend fun resolveImageUrl(directUrl: String?, fileId: String?): String? {
        directUrl?.takeIf { it.isNotBlank() }?.let {
            return productThumbnailLoadUrl(it, apiBaseUrl)
        }
        val id = fileId?.takeIf { it.isNotBlank() } ?: return null
        return resolver.resolveByFileId(id)
    }
}
