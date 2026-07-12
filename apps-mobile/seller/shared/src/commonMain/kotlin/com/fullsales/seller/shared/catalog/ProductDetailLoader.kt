package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.ProductDetail
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.StockSnapshotRepository

data class CachedProductDetail(
    val product: ProductDetail,
    val stockAvailable: Int?,
    val fromCache: Boolean,
)

/**
 * Contract (Phase 14D): Room/cache first; online enrich optional.
 */
class ProductDetailLoader(
    private val catalog: CatalogRepository,
    private val stockSnapshots: StockSnapshotRepository,
    private val apiClient: SellerApiClient,
) {
    suspend fun load(productId: String, online: Boolean): CachedProductDetail {
        val cached = catalog.getProduct(productId)?.toDetail()
        val cachedStock = stockSnapshots.get(productId)?.available
        if (!online) {
            val product = cached ?: error("PRODUCT_NOT_FOUND")
            return CachedProductDetail(product, cachedStock, fromCache = true)
        }
        val remote = runCatching { apiClient.getProduct(productId) }.getOrNull()
        val product = remote ?: cached ?: error("PRODUCT_NOT_FOUND")
        val balance = runCatching { apiClient.getStockBalance(productId).available }
            .onSuccess { stockSnapshots.put(productId, it) }
            .getOrNull()
            ?: cachedStock
        return CachedProductDetail(product, balance, fromCache = remote == null)
    }
}
