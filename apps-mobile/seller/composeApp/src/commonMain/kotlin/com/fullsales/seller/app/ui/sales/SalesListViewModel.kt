package com.fullsales.seller.app.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.sales.localSalesToListItems
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.resolveListEmptyReason
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class SalesListUiState(
    val items: List<com.fullsales.seller.shared.model.SalesListItem> = emptyList(),
    val refreshing: Boolean = false,
    val isOffline: Boolean = false,
    /** True after at least one successful sales pull (durable metadata). */
    val everSynced: Boolean = false,
    val snackbarCode: String? = null,
    val refreshFailed: Boolean = false,
) {
    val emptyReason: ListEmptyReason? get() = resolveListEmptyReason(
        hasLocalRows = items.isNotEmpty(),
        everSynced = everSynced,
        isOnline = !isOffline,
        refreshFailed = refreshFailed,
    )
}

/**
 * LocalStore-first list (Phase 16B/16F): observe Room only; online refresh runs pullSales.
 */
class SalesListViewModel(
    private val saleRepository: SaleRepository,
    private val syncCoordinator: SellerSyncCoordinator,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(SalesListUiState())
    val state: StateFlow<SalesListUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            _state.update {
                it.copy(everSynced = saleRepository.getLastSalesSyncEpochMs() != null)
            }
        }
        viewModelScope.launch {
            saleRepository.observeSales().collect { local ->
                _state.update { it.copy(items = localSalesToListItems(local)) }
            }
        }
        viewModelScope.launch {
            networkMonitor.connectivity
                .map { it == ConnectivityState.Online }
                .distinctUntilChanged()
                .collect { online ->
                    if (online && !_state.value.everSynced) {
                        refresh()
                    } else if (!online) {
                        _state.update { it.copy(isOffline = true) }
                    }
                }
        }
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun refresh() {
        viewModelScope.launch {
            if (!networkMonitor.isOnline()) {
                _state.update {
                    it.copy(
                        refreshing = false,
                        isOffline = true,
                        snackbarCode = if (it.items.isNotEmpty()) "OFFLINE" else null,
                    )
                }
                return@launch
            }
            _state.update { it.copy(refreshing = true, isOffline = false, refreshFailed = false) }
            val pulls = syncCoordinator.syncPullAndPushWithPullFlags()
            val pulled = saleRepository.getLastSalesSyncEpochMs() != null
            _state.update {
                val keepCacheFail = !pulls.salesOk && it.items.isNotEmpty()
                it.copy(
                    refreshing = false,
                    isOffline = !networkMonitor.isOnline(),
                    everSynced = pulled || it.everSynced,
                    refreshFailed = keepCacheFail,
                    snackbarCode = if (keepCacheFail) "REFRESH_FAILED" else null,
                )
            }
        }
    }
}
