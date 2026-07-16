package com.fullsales.seller.app.ui.offline

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.offline.OfflineBannerReason
import com.fullsales.seller.shared.offline.resolveOfflineBanner
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.RegistrationRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch

data class OfflineHubUiState(
    val connectivity: ConnectivityState = ConnectivityState.Connecting,
    val serverUnreachable: Boolean = false,
    val lastCatalogEpochMs: Long? = null,
    val lastSalesEpochMs: Long? = null,
    val lastRegistrationsEpochMs: Long? = null,
    val pending: List<SyncOutboxEntry> = emptyList(),
    val emptyCache: Boolean = true,
    val syncMessage: String? = null,
) {
    val statusReason: OfflineBannerReason
        get() = resolveOfflineBanner(connectivity, serverUnreachable, pending.size).reason
}

class OfflineHubViewModel(
    private val container: SellerAppContainer,
    private val catalog: CatalogRepository,
    private val sales: SaleRepository,
    private val registrations: RegistrationRepository,
    private val outbox: SyncOutboxRepository,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(OfflineHubUiState())
    val state: StateFlow<OfflineHubUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            networkMonitor.connectivity.collect { connectivity ->
                refresh(connectivity)
            }
        }
        viewModelScope.launch {
            while (isActive) {
                delay(15_000)
                refresh(networkMonitor.connectivity.value)
            }
        }
    }

    fun trySyncNow() {
        viewModelScope.launch {
            if (networkMonitor.connectivity.value == ConnectivityState.Offline) {
                _state.update { it.copy(syncMessage = "STILL_OFFLINE") }
                return@launch
            }
            _state.update { it.copy(syncMessage = null) }
            runCatching { container.syncCoordinator.syncPullAndPush() }
            refresh(networkMonitor.connectivity.value)
        }
    }

    fun clearMessage() {
        _state.update { it.copy(syncMessage = null) }
    }

    private suspend fun refresh(connectivity: ConnectivityState) {
        val pending = outbox.listPendingFifo()
        val catalogAt = catalog.getLastCatalogSyncEpochMs()
        val salesAt = sales.getLastSalesSyncEpochMs()
        val regsAt = registrations.getLastRegistrationsSyncEpochMs()
        val serverDown = if (connectivity == ConnectivityState.Online) {
            !container.apiClient.probeReachable()
        } else {
            false
        }
        _state.update {
            it.copy(
                connectivity = connectivity,
                serverUnreachable = serverDown,
                lastCatalogEpochMs = catalogAt,
                lastSalesEpochMs = salesAt,
                lastRegistrationsEpochMs = regsAt,
                pending = pending,
                emptyCache = catalogAt == null && salesAt == null,
            )
        }
    }
}
