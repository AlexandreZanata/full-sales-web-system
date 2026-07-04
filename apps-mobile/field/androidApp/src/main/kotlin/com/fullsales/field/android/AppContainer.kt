package com.fullsales.field.android

import android.content.Context
import com.fullsales.field.android.auth.TokenStore
import com.fullsales.field.android.sync.SyncWorker
import com.fullsales.field.shared.api.AuthTokenProvider
import com.fullsales.field.shared.api.FieldApiClient
import com.fullsales.field.shared.api.createFieldHttpClient
import com.fullsales.field.shared.db.FieldDatabase
import com.fullsales.field.shared.db.repository.RoomCatalogRepository
import com.fullsales.field.shared.db.repository.RoomSaleRepository
import com.fullsales.field.shared.db.repository.RoomSyncOutboxRepository
import com.fullsales.field.shared.repository.CatalogRepository
import com.fullsales.field.shared.repository.SaleRepository
import com.fullsales.field.shared.repository.SyncOutboxRepository
import com.fullsales.field.shared.sync.CatalogPullSync
import com.fullsales.field.shared.sync.FieldSyncCoordinator
import com.fullsales.field.shared.sync.OfflineSaleWriter
import com.fullsales.field.shared.sync.SyncEngine
import com.fullsales.field.shared.sync.SyncTokenRefresher

class AppContainer(context: Context) {
    private val appContext = context.applicationContext
    val tokenStore = TokenStore(appContext)
    private val database = FieldDatabase.build(appContext)
    val catalogRepository: CatalogRepository = RoomCatalogRepository(database.catalogDao())
    val saleRepository: SaleRepository = RoomSaleRepository(database.saleDao())
    val outboxRepository: SyncOutboxRepository = RoomSyncOutboxRepository(database.syncOutboxDao())
    private val tokenProvider = AuthTokenProvider { tokenStore.getAccessToken() }
    private val httpClient = createFieldHttpClient()
    private val apiClient = FieldApiClient(httpClient, tokenProvider)
    private val tokenRefresher = object : SyncTokenRefresher {
        override suspend fun refreshToken(): Boolean = false
    }
    val offlineSaleWriter = OfflineSaleWriter(saleRepository, outboxRepository)
    val syncCoordinator = FieldSyncCoordinator(
        CatalogPullSync(catalogRepository, apiClient),
        SyncEngine(outboxRepository, saleRepository, apiClient, tokenRefresher),
    )

    fun scheduleSync() {
        SyncWorker.enqueue(appContext)
    }
}
