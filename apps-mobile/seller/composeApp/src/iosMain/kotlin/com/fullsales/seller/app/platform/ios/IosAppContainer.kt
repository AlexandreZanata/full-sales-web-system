package com.fullsales.seller.app.platform.ios

import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.platform.SellerTokenStore
import com.fullsales.seller.shared.api.AuthTokenProvider
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.SellerSyncTransport
import com.fullsales.seller.shared.api.TokenRefreshHandler
import com.fullsales.seller.shared.api.createSellerHttpClient
import com.fullsales.seller.shared.auth.SellerRoleGateResult
import com.fullsales.seller.shared.auth.gateSellerAccessToken
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import com.fullsales.seller.shared.sync.CatalogPullSync
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.sync.SyncEngine
import com.fullsales.seller.shared.sync.SyncTokenRefresher

class IosAppContainer : SellerAppContainer {
    override val tokenStore: SellerTokenStore = KeychainSellerTokenStore()
    override val catalogRepository: CatalogRepository = MemoryCatalogRepository()
    override val saleRepository: SaleRepository = MemorySaleRepository()
    override val outboxRepository: SyncOutboxRepository = MemoryOutboxRepository()
    private val tokenProvider = AuthTokenProvider { tokenStore.getAccessToken() }
    private val authApiClient = SellerApiClient(createSellerHttpClient(AuthTokenProvider { null }))
    private val tokenRefresher = IosTokenRefresher(tokenStore, authApiClient)
    private val httpClient = createSellerHttpClient(tokenProvider, tokenRefresher)
    override val apiClient = SellerApiClient(httpClient)
    override val mediaUrlResolver: MediaUrlResolver = IosMediaUrlResolver(apiClient)
    private val syncTransport = SellerSyncTransport(apiClient)
    override val offlineSaleWriter = OfflineSaleWriter(saleRepository, outboxRepository)
    override val syncCoordinator = SellerSyncCoordinator(
        CatalogPullSync(catalogRepository, syncTransport),
        SyncEngine(outboxRepository, saleRepository, syncTransport, tokenRefresher),
    )
    override val networkMonitor: NetworkMonitor = IosNetworkMonitor()

    override fun requestSync() {
        // ponytail: Phase 66 wires BGTaskScheduler; foreground sync on resume.
    }

    suspend fun onAppResume() {
        syncCoordinator.syncPullAndPush()
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
) : MediaUrlResolver {
    override suspend fun resolveImageUrl(directUrl: String?, fileId: String?): String? {
        directUrl?.takeIf { it.isNotBlank() }?.let { return it }
        val id = fileId?.takeIf { it.isNotBlank() } ?: return null
        return runCatching { apiClient.getMediaUrl(id).url }.getOrNull()
    }
}

private class IosNetworkMonitor : NetworkMonitor {
    override fun isOnline(): Boolean = true
}
