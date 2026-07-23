package com.fullsales.seller.app.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.CatalogLinkShare
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.catalogBaseUrl
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.isDefinitelyOffline
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.sales.localSalesToListItems
import com.fullsales.seller.shared.share.resolveCatalogShareUrl
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.resolveListEmptyReason
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class SalesListUiState(
    val items: List<com.fullsales.seller.shared.model.SalesListItem> = emptyList(),
    val refreshing: Boolean = false,
    val isOffline: Boolean = false,
    val everSynced: Boolean = false,
    val snackbarCode: String? = null,
    val refreshFailed: Boolean = false,
    val catalogUrl: String? = null,
    val catalogShareActive: Boolean = false,
) {
    val emptyReason: ListEmptyReason? get() = resolveListEmptyReason(
        hasLocalRows = items.isNotEmpty(),
        everSynced = everSynced,
        isOnline = !isOffline,
        refreshFailed = refreshFailed,
    )
}

class SalesListViewModel(
    private val saleRepository: SaleRepository,
    private val syncCoordinator: SellerSyncCoordinator,
    private val networkMonitor: NetworkMonitor,
    private val apiClient: SellerApiClient,
) : ViewModel() {
    private val _state = MutableStateFlow(
        SalesListUiState(
            catalogUrl = catalogBaseUrl.trim().trimEnd('/').takeIf { it.isNotBlank() },
            catalogShareActive = catalogBaseUrl.isNotBlank(),
        ),
    )
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
            networkMonitor.connectivity.collect { connectivity ->
                when (connectivity) {
                    ConnectivityState.Online -> {
                        _state.update { it.copy(isOffline = false) }
                        loadCatalogShare()
                        if (!_state.value.everSynced) refresh()
                    }
                    ConnectivityState.Connecting -> {
                        _state.update { it.copy(isOffline = false) }
                    }
                    ConnectivityState.Offline -> {
                        _state.update { it.copy(isOffline = true) }
                    }
                }
            }
        }
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun shareCatalogLink() {
        val url = _state.value.catalogUrl ?: return
        CatalogLinkShare.shareText(url, "catalog")
    }

    fun copyCatalogLink() {
        val url = _state.value.catalogUrl ?: return
        CatalogLinkShare.copyToClipboard(url, "catalog")
        _state.update { it.copy(snackbarCode = "CATALOG_COPIED") }
    }

    fun openCatalogLink() {
        val url = _state.value.catalogUrl ?: return
        CatalogLinkShare.openUrl(url)
    }

    fun reloadCatalogShare() {
        viewModelScope.launch { loadCatalogShare() }
    }

    fun refresh() {
        viewModelScope.launch {
            if (!networkMonitor.canAttemptNetwork()) {
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
            loadCatalogShare()
            val pulls = syncCoordinator.syncPullAndPushWithPullFlags()
            val pulled = saleRepository.getLastSalesSyncEpochMs() != null
            _state.update {
                val keepCacheFail = !pulls.salesOk && it.items.isNotEmpty()
                it.copy(
                    refreshing = false,
                    isOffline = networkMonitor.connectivity.value.isDefinitelyOffline(),
                    everSynced = pulled || it.everSynced,
                    refreshFailed = keepCacheFail,
                    snackbarCode = if (keepCacheFail) "REFRESH_FAILED" else null,
                )
            }
        }
    }

    private suspend fun loadCatalogShare() {
        val fallback = catalogBaseUrl.trim().trimEnd('/').takeIf { it.isNotBlank() }
        if (!networkMonitor.canAttemptNetwork()) {
            _state.update {
                it.copy(
                    catalogUrl = fallback ?: it.catalogUrl,
                    catalogShareActive = (fallback ?: it.catalogUrl) != null,
                )
            }
            return
        }
        runCatching { apiClient.getSellerShare() }
            .onSuccess { share ->
                val url = resolveCatalogShareUrl(share)
                _state.update {
                    it.copy(catalogUrl = url, catalogShareActive = url != null)
                }
            }
            .onFailure {
                _state.update {
                    it.copy(catalogUrl = fallback, catalogShareActive = fallback != null)
                }
            }
    }
}
