package com.fullsales.seller.app.ui.products

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.catalog.StockBalancePrefetcher
import com.fullsales.seller.shared.catalog.filterProductsBySearch
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sales.filterProductsAvailableForBrowsing
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
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
    val error: String? = null,
    val snackbarCode: String? = null,
    val isOffline: Boolean = false,
) {
    val filtered: List<Product> = filterProductsAvailableForBrowsing(
        filterProductsBySearch(items, searchQuery),
        stockByProductId,
    )

    val isEmpty: Boolean get() = !refreshing && error == null && filtered.isEmpty()
}

class ProductViewModel(
    private val catalogRepository: CatalogRepository,
    private val syncCoordinator: SellerSyncCoordinator,
    private val networkMonitor: NetworkMonitor,
    private val stockPrefetcher: StockBalancePrefetcher,
) : ViewModel() {
    private val _searchQuery = MutableStateFlow("")
    private val _refreshing = MutableStateFlow(false)
    private val _error = MutableStateFlow<String?>(null)
    private val _stockByProductId = MutableStateFlow<Map<String, Int>>(emptyMap())
    private val _state = MutableStateFlow(ProductListUiState())
    val state: StateFlow<ProductListUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            _stockByProductId.value = stockPrefetcher.cachedMap()
        }
        viewModelScope.launch {
            combine(
                catalogRepository.observeProducts(),
                _searchQuery,
                _refreshing,
                _error,
                _stockByProductId,
            ) { products, query, refreshing, error, stockByProductId ->
                ProductListUiState(
                    items = products,
                    searchQuery = query,
                    stockByProductId = stockByProductId,
                    refreshing = refreshing,
                    error = error,
                    snackbarCode = _state.value.snackbarCode,
                    isOffline = !networkMonitor.isOnline(),
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
            if (!networkMonitor.isOnline()) {
                _state.update { it.copy(snackbarCode = "OFFLINE", refreshing = false) }
                return@launch
            }
            _refreshing.value = true
            _error.value = null
            runCatching { syncCoordinator.syncPullAndPush() }
                .onFailure { _error.value = it.message ?: "Sync failed" }
            _refreshing.value = false
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
