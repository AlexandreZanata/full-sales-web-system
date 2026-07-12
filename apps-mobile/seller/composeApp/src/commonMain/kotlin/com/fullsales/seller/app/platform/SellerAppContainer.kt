package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import kotlinx.coroutines.flow.StateFlow

interface SellerTokenStore {
    fun getAccessToken(): String?
    fun getRefreshToken(): String?
    fun saveTokens(accessToken: String, refreshToken: String)
    fun clear()
}

interface MediaUrlResolver {
    suspend fun resolveImageUrl(directUrl: String?, fileId: String?): String?
}

interface NetworkMonitor {
    val connectivity: StateFlow<ConnectivityState>
    fun isOnline(): Boolean = connectivity.value == ConnectivityState.Online
}

interface SellerAppContainer {
    val apiClient: SellerApiClient
    val tokenStore: SellerTokenStore
    val catalogRepository: CatalogRepository
    val saleRepository: SaleRepository
    val outboxRepository: SyncOutboxRepository
    val syncCoordinator: SellerSyncCoordinator
    val offlineSaleWriter: OfflineSaleWriter
    val mediaUrlResolver: MediaUrlResolver
    val networkMonitor: NetworkMonitor
    fun requestSync()
}

expect class LocaleStore() {
    fun read(): com.fullsales.seller.shared.i18n.SellerLocale
    fun write(locale: com.fullsales.seller.shared.i18n.SellerLocale)
}

expect fun createNetworkMonitor(): NetworkMonitor

expect fun isDebugBuild(): Boolean
