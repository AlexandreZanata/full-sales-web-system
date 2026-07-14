package com.fullsales.seller.android

import android.content.Context
import com.fullsales.seller.android.auth.SellerTokenRefresher
import com.fullsales.seller.android.auth.TokenStore
import com.fullsales.seller.android.media.MediaUrlCache
import com.fullsales.seller.android.sync.SyncWorker
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.platform.SellerTokenStore
import com.fullsales.seller.app.platform.createNetworkMonitor
import com.fullsales.seller.app.platform.initAndroidPlatform
import com.fullsales.seller.shared.api.AuthTokenProvider
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.SellerSyncTransport
import com.fullsales.seller.shared.api.createSellerHttpClient
import com.fullsales.seller.shared.db.SellerDatabase
import com.fullsales.seller.shared.db.repository.RoomCatalogRepository
import com.fullsales.seller.shared.db.repository.RoomCommerceAddressCache
import com.fullsales.seller.shared.db.repository.RoomRegistrationRepository
import com.fullsales.seller.shared.db.repository.RoomSaleRepository
import com.fullsales.seller.shared.db.repository.RoomStockSnapshotRepository
import com.fullsales.seller.shared.db.repository.RoomSyncOutboxRepository
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.CommerceAddressCache
import com.fullsales.seller.shared.repository.RegistrationRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.StockSnapshotRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import com.fullsales.seller.shared.connectivity.OnlineSyncTrigger
import com.fullsales.seller.shared.sync.CatalogPullSync
import com.fullsales.seller.shared.sync.OfflineRegistrationWriter
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import com.fullsales.seller.shared.sync.PullRegistrationsSync
import com.fullsales.seller.shared.sync.PullSalesSync
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.sync.SyncEngine
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob

class AppContainer(context: Context) : SellerAppContainer {
    init {
        initAndroidPlatform(context)
    }

    private val androidContext = context.applicationContext
    private val appScope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val database = SellerDatabase.build(androidContext)
    private val tokenStoreImpl = TokenStore(androidContext)
    override val tokenStore: SellerTokenStore = tokenStoreImpl
    override val catalogRepository: CatalogRepository = RoomCatalogRepository(database.catalogDao())
    override val saleRepository: SaleRepository =
        RoomSaleRepository(database.saleDao(), database.catalogDao())
    override val outboxRepository: SyncOutboxRepository = RoomSyncOutboxRepository(database.syncOutboxDao())
    override val stockSnapshots: StockSnapshotRepository =
        RoomStockSnapshotRepository(database.cacheDao())
    override val commerceAddressCache: CommerceAddressCache =
        RoomCommerceAddressCache(database.cacheDao())
    private val tokenProvider = AuthTokenProvider { tokenStoreImpl.getAccessToken() }
    private val authHttpClient = createSellerHttpClient(AuthTokenProvider { null })
    private val authApiClient = SellerApiClient(authHttpClient)
    private val tokenRefresher = SellerTokenRefresher(tokenStoreImpl, authApiClient)
    private val httpClient = createSellerHttpClient(tokenProvider, tokenRefresher)
    override val apiClient = SellerApiClient(httpClient)
    private val mediaUrlCacheImpl = MediaUrlCache(apiClient)
    override val mediaUrlResolver: MediaUrlResolver = mediaUrlCacheImpl
    private val syncTransport = SellerSyncTransport(apiClient)
    override val offlineSaleWriter = OfflineSaleWriter(saleRepository, outboxRepository)
    override val registrationRepository: RegistrationRepository =
        RoomRegistrationRepository(database.registrationDao(), database.catalogDao())
    override val offlineRegistrationWriter =
        OfflineRegistrationWriter(registrationRepository, outboxRepository)
    override val syncCoordinator = SellerSyncCoordinator(
        CatalogPullSync(catalogRepository, syncTransport),
        PullSalesSync(saleRepository, syncTransport),
        PullRegistrationsSync(registrationRepository, syncTransport),
        SyncEngine(
            outboxRepository,
            saleRepository,
            syncTransport,
            tokenRefresher,
            registrationRepository,
        ),
    )
    override val networkMonitor: NetworkMonitor = createNetworkMonitor()

    init {
        OnlineSyncTrigger(
            networkMonitor.connectivity,
            syncCoordinator::pushOutbox,
            appScope,
        )
    }

    fun scheduleSync() {
        SyncWorker.enqueuePeriodic(androidContext)
    }

    override fun requestSync() {
        SyncWorker.enqueueOneTime(androidContext)
    }
}
