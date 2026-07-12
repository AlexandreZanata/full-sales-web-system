package com.fullsales.seller.app.ui.sync

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch

enum class SyncBadge {
    Idle,
    Offline,
    Syncing,
    SyncFailed,
}

class SyncStatusViewModel(
    private val container: SellerAppContainer,
    private val sales: SaleRepository,
    private val outbox: SyncOutboxRepository,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _badge = MutableStateFlow(SyncBadge.Idle)
    val badge: StateFlow<SyncBadge> = _badge.asStateFlow()
    private val _refreshing = MutableStateFlow(false)
    val refreshing: StateFlow<Boolean> = _refreshing.asStateFlow()
    private var latestSales: List<LocalSale> = emptyList()

    init {
        viewModelScope.launch {
            combine(
                sales.observeSales(),
                networkMonitor.connectivity,
            ) { localSales, connectivity -> localSales to connectivity }
                .collect { (localSales, connectivity) ->
                    latestSales = localSales
                    updateBadge(localSales, connectivity)
                }
        }
    }

    fun refreshNow() {
        viewModelScope.launch {
            _refreshing.value = true
            _badge.value = SyncBadge.Syncing
            runCatching { container.syncCoordinator.syncPullAndPush() }
            _refreshing.value = false
            updateBadge(latestSales, networkMonitor.connectivity.value)
        }
    }

    private suspend fun updateBadge(
        localSales: List<LocalSale>,
        connectivity: ConnectivityState,
    ) {
        _badge.value = when {
            _refreshing.value -> SyncBadge.Syncing
            localSales.any { it.status == LocalSaleStatus.SyncFailed } -> SyncBadge.SyncFailed
            connectivity != ConnectivityState.Online -> SyncBadge.Offline
            outbox.listPendingFifo().isNotEmpty() -> SyncBadge.Syncing
            else -> SyncBadge.Idle
        }
    }
}
