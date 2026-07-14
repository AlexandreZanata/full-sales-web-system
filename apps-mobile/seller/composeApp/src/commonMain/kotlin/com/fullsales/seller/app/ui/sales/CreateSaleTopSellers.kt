package com.fullsales.seller.app.ui.sales

import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.connectivity.allowsInternetOnlyActions
import com.fullsales.seller.shared.model.TopSellingProduct
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

/** OD-16-6 = B: hide top-seller chips when offline or missing. */
internal fun CoroutineScope.observeTopSellersOfflineHide(
    networkMonitor: NetworkMonitor,
    apiClient: SellerApiClient,
    state: MutableStateFlow<CreateSaleUiState>,
    onFetched: (List<TopSellingProduct>) -> Unit,
) {
    launch {
        networkMonitor.connectivity.collect { connectivity ->
            if (!connectivity.allowsInternetOnlyActions()) {
                state.update { it.copy(topSellingProducts = emptyList()) }
            } else {
                loadTopSellingProducts(networkMonitor, apiClient, state, onFetched)
            }
        }
    }
}

private fun CoroutineScope.loadTopSellingProducts(
    networkMonitor: NetworkMonitor,
    apiClient: SellerApiClient,
    state: MutableStateFlow<CreateSaleUiState>,
    onFetched: (List<TopSellingProduct>) -> Unit,
) {
    launch {
        if (!networkMonitor.isOnline()) return@launch
        runCatching { apiClient.listTopSellingProducts(limit = 5) }
            .onSuccess { response ->
                state.update { it.copy(topSellingProducts = response.data) }
                onFetched(response.data)
            }
    }
}
