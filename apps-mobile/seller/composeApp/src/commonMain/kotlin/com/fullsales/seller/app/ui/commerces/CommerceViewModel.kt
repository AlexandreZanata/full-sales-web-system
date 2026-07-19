package com.fullsales.seller.app.ui.commerces

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.connectivity.isDefinitelyOffline
import com.fullsales.seller.shared.catalog.filterCommercesBySearch
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.resolveListEmptyReason
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class CommerceListUiState(
    val items: List<Commerce> = emptyList(),
    val searchQuery: String = "",
    val activeOnly: Boolean = true,
    val refreshing: Boolean = false,
    val isOffline: Boolean = false,
    val everSynced: Boolean = false,
    val snackbarCode: String? = null,
    val refreshFailed: Boolean = false,
) {
    val filtered: List<Commerce> = filterCommercesBySearch(
        items.filter { !activeOnly || it.active },
        searchQuery,
    )

    val emptyReason: ListEmptyReason? get() = resolveListEmptyReason(
        hasLocalRows = items.isNotEmpty(),
        everSynced = everSynced,
        isOnline = !isOffline,
        refreshFailed = refreshFailed,
    )

    val isFilterEmpty: Boolean get() = !refreshing && items.isNotEmpty() && filtered.isEmpty()
}

class CommerceViewModel(
    private val catalogRepository: CatalogRepository,
    private val syncCoordinator: SellerSyncCoordinator,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _searchQuery = MutableStateFlow("")
    private val _activeOnly = MutableStateFlow(true)
    private val _refreshing = MutableStateFlow(false)
    private val _everSynced = MutableStateFlow(false)
    private val _refreshFailed = MutableStateFlow(false)
    private val _state = MutableStateFlow(CommerceListUiState())
    val state: StateFlow<CommerceListUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            _everSynced.value = catalogRepository.getLastCatalogSyncEpochMs() != null
        }
        viewModelScope.launch {
            combine(
                catalogRepository.observeCommerces(),
                _searchQuery,
                _activeOnly,
                _refreshing,
                _everSynced,
            ) { commerces, query, activeOnly, refreshing, everSynced ->
                CommerceListUiState(
                    items = commerces,
                    searchQuery = query,
                    activeOnly = activeOnly,
                    refreshing = refreshing,
                    isOffline = networkMonitor.connectivity.value.isDefinitelyOffline(),
                    everSynced = everSynced,
                    snackbarCode = _state.value.snackbarCode,
                    refreshFailed = _refreshFailed.value,
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
}
