package com.fullsales.seller.app.ui.products

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.connectivity.isDefinitelyOffline
import com.fullsales.seller.shared.catalog.StockBalancePrefetcher
import com.fullsales.seller.shared.catalog.filterProductsBySearch
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sales.filterProductsAvailableForBrowsing
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.resolveListEmptyReason
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class ProductListUiState(
    val items: List<Product> = emptyList(),
    val searchQuery: String = "",
    val stockByProductId: Map<String, Int> = emptyMap(),
    val refreshing: Boolean = false,
    val snackbarCode: String? = null,
    val isOffline: Boolean = false,
    val everSynced: Boolean = false,
    val refreshFailed: Boolean = false,
) {
    val filtered: List<Product> = filterProductsAvailableForBrowsing(
        filterProductsBySearch(items, searchQuery),
        stockByProductId,
    )

    val emptyReason: ListEmptyReason? get() = resolveListEmptyReason(
        hasLocalRows = items.isNotEmpty(),
        everSynced = everSynced,
        isOnline = !isOffline,
        refreshFailed = refreshFailed,
    )

    val isFilterEmpty: Boolean get() = !refreshing && items.isNotEmpty() && filtered.isEmpty()
}

class ProductViewModel(
    private val catalogRepository: CatalogRepository,
    private val syncCoordinator: SellerSyncCoordinator,
    private val networkMonitor: NetworkMonitor,
    private val stockPrefetcher: StockBalancePrefetcher,
) : ViewModel() {
    private val _searchQuery = MutableStateFlow("")
    private val _refreshing = MutableStateFlow(false)
    private val _everSynced = MutableStateFlow(false)
    private val _refreshFailed = MutableStateFlow(false)
    private val _stockByProductId = MutableStateFlow<Map<String, Int>>(emptyMap())
    private val _state = MutableStateFlow(ProductListUiState())
    val state: StateFlow<ProductListUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            _stockByProductId.value = stockPrefetcher.cachedMap()
            _everSynced.value = catalogRepository.getLastCatalogSyncEpochMs() != null
        }
        viewModelScope.launch {
            combine(
                catalogRepository.observeProducts(),
                _searchQuery,
                _refreshing,
                _stockByProductId,
                _everSynced,
            ) { products, query, refreshing, stockByProductId, everSynced ->
                ProductListUiState(
                    items = products,
                    searchQuery = query,
                    stockByProductId = stockByProductId,
                    refreshing = refreshing,
                    snackbarCode = _state.value.snackbarCode,
                    isOffline = networkMonitor.connectivity.value.isDefinitelyOffline(),
                    everSynced = everSynced,
                    refreshFailed = _refreshFailed.value,
                )
            }.collect { _state.value = it }
        }
        viewModelScope.launch {
            catalogRepository.observeProducts().collect { products ->
                prefetchStockBalances(products.map { it.id })
            }
        }
    }

    fun setSearchQuery(query: String) {
        _searchQuery.value = query
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun refresh() {
        viewModelScope.launch {
            if (!networkMonitor.canAttemptNetwork()) {
                _state.update {
                    it.copy(snackbarCode = if (it.items.isNotEmpty()) "OFFLINE" else null, refreshing = false)
                }
                return@launch
            }
            _refreshing.value = true
            _refreshFailed.value = false
            val pulls = syncCoordinator.syncPullAndPushWithPullFlags()
            val synced = catalogRepository.getLastCatalogSyncEpochMs() != null
            _everSynced.value = synced || _everSynced.value
            val keepCacheFail = !pulls.catalogOk && _state.value.items.isNotEmpty()
            _refreshFailed.value = keepCacheFail
            _refreshing.value = false
            if (keepCacheFail) {
                _state.update { it.copy(snackbarCode = "REFRESH_FAILED", refreshFailed = true) }
            }
        }
    }

    private fun prefetchStockBalances(productIds: List<String>) {
        if (!networkMonitor.isOnline()) return
        productIds.filter { it.isNotBlank() && it !in _stockByProductId.value }
            .forEach { productId ->
                viewModelScope.launch {
                    stockPrefetcher.fetchAndCache(productId)?.let { available ->
                        _stockByProductId.update { it + (productId to available) }
                    }
                }
            }
    }
}
