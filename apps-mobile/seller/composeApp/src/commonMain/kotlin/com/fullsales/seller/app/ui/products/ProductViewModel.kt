package com.fullsales.seller.app.ui.products

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.shared.catalog.filterProductsBySearch
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch

data class ProductListUiState(
    val items: List<Product> = emptyList(),
    val searchQuery: String = "",
    val refreshing: Boolean = false,
    val error: String? = null,
) {
    val filtered: List<Product> = filterProductsBySearch(items, searchQuery)

    val isEmpty: Boolean get() = !refreshing && error == null && filtered.isEmpty()
}

class ProductViewModel(
    private val catalogRepository: CatalogRepository,
    private val syncCoordinator: SellerSyncCoordinator,
) : ViewModel() {
    private val _searchQuery = MutableStateFlow("")
    private val _refreshing = MutableStateFlow(false)
    private val _error = MutableStateFlow<String?>(null)
    private val _state = MutableStateFlow(ProductListUiState())
    val state: StateFlow<ProductListUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            combine(
                catalogRepository.observeProducts(),
                _searchQuery,
                _refreshing,
                _error,
            ) { products, query, refreshing, error ->
                ProductListUiState(
                    items = products,
                    searchQuery = query,
                    refreshing = refreshing,
                    error = error,
                )
            }.collect { _state.value = it }
        }
    }

    fun setSearchQuery(query: String) {
        _searchQuery.value = query
    }

    fun refresh() {
        viewModelScope.launch {
            _refreshing.value = true
            _error.value = null
            runCatching { syncCoordinator.syncPullAndPush() }
                .onFailure { _error.value = it.message ?: "Sync failed" }
            _refreshing.value = false
        }
    }
}
