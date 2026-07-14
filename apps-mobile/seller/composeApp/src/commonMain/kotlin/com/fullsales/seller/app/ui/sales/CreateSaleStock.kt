package com.fullsales.seller.app.ui.sales

import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.catalog.StockBalancePrefetcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

internal fun CoroutineScope.loadCreateSaleStock(
    productId: String,
    networkMonitor: NetworkMonitor,
    stockPrefetcher: StockBalancePrefetcher,
    state: MutableStateFlow<CreateSaleUiState>,
) {
    launch {
        if (!networkMonitor.isOnline()) {
            stockPrefetcher.cachedMap()[productId]?.let { available ->
                state.update { it.copy(stockByProductId = it.stockByProductId + (productId to available)) }
            }
            return@launch
        }
        stockPrefetcher.fetchAndCache(productId)?.let { available ->
            state.update { it.copy(stockByProductId = it.stockByProductId + (productId to available)) }
        }
    }
}
