package com.fullsales.seller.app.ui.sync

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.offline.OfflineBannerState
import com.fullsales.seller.shared.offline.resolveOfflineBanner
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch

enum class SyncBadge {
    Online,
    Offline,
    Connecting,
    Syncing,
    SyncFailed,
}

/** Online/Connecting stay invisible — header only warns when offline or sync needs attention. */
fun SyncBadge.shouldShowInHeader(): Boolean = when (this) {
    SyncBadge.Online, SyncBadge.Connecting -> false
    SyncBadge.Offline, SyncBadge.Syncing, SyncBadge.SyncFailed -> true
}

class SyncStatusViewModel(
    private val container: SellerAppContainer,
    private val sales: SaleRepository,
    private val outbox: SyncOutboxRepository,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _badge = MutableStateFlow(SyncBadge.Connecting)
    val badge: StateFlow<SyncBadge> = _badge.asStateFlow()
    private val _refreshing = MutableStateFlow(false)
    val refreshing: StateFlow<Boolean> = _refreshing.asStateFlow()
    private val _offlineBanner = MutableStateFlow(
        OfflineBannerState(visible = false, reason = com.fullsales.seller.shared.offline.OfflineBannerReason.None, pendingCount = 0),
    )
    val offlineBanner: StateFlow<OfflineBannerState> = _offlineBanner.asStateFlow()
    private var latestSales: List<LocalSale> = emptyList()
    private var serverUnreachable: Boolean = false

    init {
        viewModelScope.launch {
            networkMonitor.connectivity.collect { connectivity ->
                updateBadge(latestSales, connectivity)
                refreshBanner(connectivity)
            }
        }
        viewModelScope.launch {
            sales.observeSales().collect { localSales ->
                latestSales = localSales
                updateBadge(localSales, networkMonitor.connectivity.value)
            }
        }
        viewModelScope.launch {
            while (isActive) {
                delay(15_000)
                refreshBanner(networkMonitor.connectivity.value)
            }
        }
    }

    fun refreshNow() {
        viewModelScope.launch {
            when (networkMonitor.connectivity.value) {
                ConnectivityState.Offline -> {
                    _badge.value = SyncBadge.Offline
                    refreshBanner(ConnectivityState.Offline)
                    return@launch
                }
                ConnectivityState.Connecting -> {
                    _badge.value = SyncBadge.Connecting
                    return@launch
                }
                ConnectivityState.Online -> Unit
            }
            _refreshing.value = true
            _badge.value = SyncBadge.Syncing
            runCatching { container.syncCoordinator.syncPullAndPush() }
            _refreshing.value = false
            updateBadge(latestSales, networkMonitor.connectivity.value)
            refreshBanner(networkMonitor.connectivity.value)
        }
    }

    private suspend fun refreshBanner(connectivity: ConnectivityState) {
        when (connectivity) {
            ConnectivityState.Online -> {
                serverUnreachable = !container.apiClient.probeReachable()
            }
            ConnectivityState.Connecting -> {
                serverUnreachable = false
            }
            ConnectivityState.Offline -> {
                // Device Offline ≠ API down (guia ADR-007). Health success promotes Online.
                serverUnreachable = false
                if (container.apiClient.probeReachable()) {
                    networkMonitor.reportPathReachable()
                }
            }
        }
        val pending = outbox.listPendingFifo().size
        _offlineBanner.value = resolveOfflineBanner(
            networkMonitor.connectivity.value,
            serverUnreachable,
            pending,
        )
    }

    private suspend fun updateBadge(
        localSales: List<LocalSale>,
        connectivity: ConnectivityState,
    ) {
        _badge.value = when {
            connectivity == ConnectivityState.Offline -> SyncBadge.Offline
            _refreshing.value -> SyncBadge.Syncing
            localSales.any { it.status == LocalSaleStatus.SyncFailed } -> SyncBadge.SyncFailed
            connectivity == ConnectivityState.Connecting -> SyncBadge.Connecting
            outbox.listPendingFifo().isNotEmpty() -> SyncBadge.Syncing
            else -> SyncBadge.Online
        }
    }
}
