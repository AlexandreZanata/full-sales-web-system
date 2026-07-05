package com.fullsales.seller.android

import android.content.Context
import com.fullsales.seller.android.auth.SellerTokenRefresher
import com.fullsales.seller.android.auth.TokenStore
import com.fullsales.seller.android.media.MediaUrlCache
import com.fullsales.seller.android.sync.SyncWorker
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
import com.fullsales.seller.shared.sync.CatalogPullSync
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.sync.SyncEngine

class AppContainer(context: Context) {
    val appContext = context.applicationContext
    val tokenStore = TokenStore(appContext)
    private val database = SellerDatabase.build(appContext)
    val catalogRepository: CatalogRepository = RoomCatalogRepository(database.catalogDao())
    val saleRepository: SaleRepository = RoomSaleRepository(database.saleDao())
    val outboxRepository: SyncOutboxRepository = RoomSyncOutboxRepository(database.syncOutboxDao())
    private val tokenProvider = AuthTokenProvider { tokenStore.getAccessToken() }
    private val authHttpClient = createSellerHttpClient(AuthTokenProvider { null })
    private val authApiClient = SellerApiClient(authHttpClient)
    private val tokenRefresher = SellerTokenRefresher(tokenStore, authApiClient)
    private val httpClient = createSellerHttpClient(tokenProvider, tokenRefresher)
    val apiClient = SellerApiClient(httpClient)
    val mediaUrlCache = MediaUrlCache(apiClient)
    private val syncTransport = SellerSyncTransport(apiClient)
    val offlineSaleWriter = OfflineSaleWriter(saleRepository, outboxRepository)
    val syncCoordinator = SellerSyncCoordinator(
        CatalogPullSync(catalogRepository, syncTransport),
        SyncEngine(outboxRepository, saleRepository, syncTransport, tokenRefresher),
    )

    fun scheduleSync() {
        SyncWorker.enqueuePeriodic(appContext)
    }

    fun requestSync() {
        SyncWorker.enqueueOneTime(appContext)
    }
}
