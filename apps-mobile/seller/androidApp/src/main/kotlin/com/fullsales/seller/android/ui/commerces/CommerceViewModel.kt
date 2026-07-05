package com.fullsales.seller.android.ui.commerces

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch

data class CommerceListUiState(
    val items: List<Commerce> = emptyList(),
    val searchQuery: String = "",
    val activeOnly: Boolean = true,
    val refreshing: Boolean = false,
    val error: String? = null,
) {
    val filtered: List<Commerce> = items
        .asSequence()
        .filter { !activeOnly || it.active }
        .filter { commerceMatchesSearch(it, searchQuery) }
        .sortedBy { it.displayName().lowercase() }
        .toList()

    val isEmpty: Boolean get() = !refreshing && error == null && filtered.isEmpty()
}

private fun commerceMatchesSearch(commerce: Commerce, query: String): Boolean {
    val term = query.trim().lowercase()
    if (term.isEmpty()) return true
    return commerce.legalName.lowercase().contains(term) ||
        commerce.tradeName?.lowercase()?.contains(term) == true
}

class CommerceViewModel(
    private val catalogRepository: CatalogRepository,
    private val syncCoordinator: SellerSyncCoordinator,
) : ViewModel() {
    private val _searchQuery = MutableStateFlow("")
    private val _activeOnly = MutableStateFlow(true)
    private val _refreshing = MutableStateFlow(false)
    private val _error = MutableStateFlow<String?>(null)
    private val _state = MutableStateFlow(CommerceListUiState())
    val state: StateFlow<CommerceListUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            combine(
                catalogRepository.observeCommerces(),
                _searchQuery,
                _activeOnly,
                _refreshing,
                _error,
            ) { commerces, query, activeOnly, refreshing, error ->
                CommerceListUiState(
                    items = commerces,
                    searchQuery = query,
                    activeOnly = activeOnly,
                    refreshing = refreshing,
                    error = error,
                )
            }.collect { _state.value = it }
        }
    }

    fun setSearchQuery(query: String) {
        _searchQuery.value = query
    }

    fun setActiveOnly(activeOnly: Boolean) {
        _activeOnly.value = activeOnly
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
