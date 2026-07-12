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
import com.fullsales.seller.shared.db.repository.RoomSaleRepository
import com.fullsales.seller.shared.db.repository.RoomSyncOutboxRepository
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import com.fullsales.seller.shared.connectivity.OnlineSyncTrigger
import com.fullsales.seller.shared.sync.CatalogPullSync
import com.fullsales.seller.shared.sync.OfflineSaleWriter
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
    override val saleRepository: SaleRepository = RoomSaleRepository(database.saleDao())
    override val outboxRepository: SyncOutboxRepository = RoomSyncOutboxRepository(database.syncOutboxDao())
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
    override val syncCoordinator = SellerSyncCoordinator(
        CatalogPullSync(catalogRepository, syncTransport),
        SyncEngine(outboxRepository, saleRepository, syncTransport, tokenRefresher),
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
