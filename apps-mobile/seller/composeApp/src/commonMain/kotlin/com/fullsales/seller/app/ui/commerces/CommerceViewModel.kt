package com.fullsales.seller.app.ui.commerces

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.ApiException
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
    val errorCode: String? = null,
    val isOffline: Boolean = false,
    val snackbarCode: String? = null,
) {
    val filtered: List<Commerce> = items
        .asSequence()
        .filter { !activeOnly || it.active }
        .filter { commerceMatchesSearch(it, searchQuery) }
        .sortedBy { it.displayName().lowercase() }
        .toList()

    val isEmpty: Boolean get() = !refreshing && errorCode == null && filtered.isEmpty()
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
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _searchQuery = MutableStateFlow("")
    private val _activeOnly = MutableStateFlow(true)
    private val _refreshing = MutableStateFlow(false)
    private val _errorCode = MutableStateFlow<String?>(null)
    private val _state = MutableStateFlow(CommerceListUiState())
    val state: StateFlow<CommerceListUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            combine(
                catalogRepository.observeCommerces(),
                _searchQuery,
                _activeOnly,
                _refreshing,
                _errorCode,
            ) { commerces, query, activeOnly, refreshing, errorCode ->
                CommerceListUiState(
                    items = commerces,
                    searchQuery = query,
                    activeOnly = activeOnly,
                    refreshing = refreshing,
                    errorCode = errorCode,
                    isOffline = !networkMonitor.isOnline(),
                    snackbarCode = _state.value.snackbarCode,
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

    fun clearSnackbar() {
        _state.value = _state.value.copy(snackbarCode = null)
    }

    fun refresh() {
        viewModelScope.launch {
            if (!networkMonitor.isOnline()) {
                _state.value = _state.value.copy(snackbarCode = "OFFLINE", refreshing = false)
                return@launch
            }
            _refreshing.value = true
            _errorCode.value = null
            runCatching { syncCoordinator.syncPullAndPush() }
                .onSuccess { _errorCode.value = null }
                .onFailure { err -> _errorCode.value = mapSyncError(err) }
            _refreshing.value = false
        }
    }

    private fun mapSyncError(error: Throwable): String = when (error) {
        is ApiException -> error.detail.code
        else -> "SYNC_FAILED"
    }
}
