package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.repository.StockSnapshotRepository

/**
 * Hydrates UI stock maps from disk cache; refreshes from API when online.
 */
class StockBalancePrefetcher(
    private val apiClient: SellerApiClient,
    private val snapshots: StockSnapshotRepository,
) {
    suspend fun cachedMap(): Map<String, Int> = snapshots.getAvailableMap()

    suspend fun fetchAndCache(productId: String): Int? =
        runCatching { apiClient.getStockBalance(productId).available }
            .onSuccess { snapshots.put(productId, it) }
            .getOrNull()
}
