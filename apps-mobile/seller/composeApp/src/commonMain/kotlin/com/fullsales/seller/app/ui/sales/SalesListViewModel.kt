package com.fullsales.seller.app.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SalesListItem
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.sales.mergeSalesList
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class SalesListUiState(
    val items: List<SalesListItem> = emptyList(),
    val refreshing: Boolean = false,
    val isOffline: Boolean = false,
    val remoteLoaded: Boolean = false,
    val snackbarCode: String? = null,
)

class SalesListViewModel(
    private val apiClient: SellerApiClient,
    private val saleRepository: SaleRepository,
    private val syncCoordinator: SellerSyncCoordinator,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(SalesListUiState())
    val state: StateFlow<SalesListUiState> = _state.asStateFlow()
    private var remoteSales: List<Sale> = emptyList()
    private var localSales: List<com.fullsales.seller.shared.model.LocalSale> = emptyList()

    init {
        viewModelScope.launch {
            saleRepository.observeSales().collect { local ->
                localSales = local
                publishMerged()
            }
        }
        refresh()
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun refresh() {
        viewModelScope.launch {
            if (!networkMonitor.isOnline()) {
                _state.update {
                    it.copy(refreshing = false, isOffline = true, snackbarCode = "OFFLINE")
                }
                return@launch
            }
            _state.update { it.copy(refreshing = true, isOffline = false) }
            runCatching { syncCoordinator.syncPullAndPush() }
            val fetchOk = runCatching {
                remoteSales = apiClient.listSales(limit = 20).data
            }.isSuccess
            _state.update {
                it.copy(
                    refreshing = false,
                    isOffline = !networkMonitor.isOnline() || !fetchOk,
                    remoteLoaded = fetchOk || it.remoteLoaded,
                )
            }
            publishMerged()
        }
    }

    private fun publishMerged() {
        _state.update {
            it.copy(items = mergeSalesList(remoteSales, localSales))
        }
    }
}
