package com.fullsales.seller.android.ui.sales

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
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
)

class SalesListViewModel(
    private val apiClient: SellerApiClient,
    private val saleRepository: SaleRepository,
    private val syncCoordinator: SellerSyncCoordinator,
    context: Context,
) : ViewModel() {
    private val connectivity = context.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
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

    fun refresh() {
        viewModelScope.launch {
            _state.update { it.copy(refreshing = true, isOffline = !isOnline()) }
            runCatching { syncCoordinator.syncPullAndPush() }
            val fetchOk = runCatching {
                remoteSales = apiClient.listSales(page = 1, pageSize = 20).items
            }.isSuccess
            _state.update {
                it.copy(
                    refreshing = false,
                    isOffline = !isOnline() || !fetchOk,
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

    private fun isOnline(): Boolean {
        val network = connectivity.activeNetwork ?: return false
        val caps = connectivity.getNetworkCapabilities(network) ?: return false
        return caps.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }
}
