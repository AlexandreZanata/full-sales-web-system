package com.fullsales.seller.android

import android.content.Context
import com.fullsales.seller.android.auth.NoOpTokenRefresher
import com.fullsales.seller.android.auth.TokenStore
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
    private val httpClient = createSellerHttpClient(tokenProvider)
    val apiClient = SellerApiClient(httpClient)
    private val syncTransport = SellerSyncTransport(apiClient)
    val offlineSaleWriter = OfflineSaleWriter(saleRepository, outboxRepository)
    val syncCoordinator = SellerSyncCoordinator(
        CatalogPullSync(catalogRepository, syncTransport),
        SyncEngine(outboxRepository, saleRepository, syncTransport, NoOpTokenRefresher()),
    )

    fun scheduleSync() {
        SyncWorker.enqueuePeriodic(appContext)
    }

    fun requestSync() {
        SyncWorker.enqueueOneTime(appContext)
    }
}
