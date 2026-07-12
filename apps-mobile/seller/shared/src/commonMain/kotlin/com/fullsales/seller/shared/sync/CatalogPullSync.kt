package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CursorListCommerces
import com.fullsales.seller.shared.model.CursorListProducts
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.CatalogRepository

interface CatalogPullClient {
    suspend fun fetchCommerces(limit: Int, cursor: String?): CursorListCommerces
    suspend fun fetchProducts(limit: Int, cursor: String?): CursorListProducts
}

class CatalogPullSync(
    private val catalog: CatalogRepository,
    private val client: CatalogPullClient,
    private val pageSize: Int = 50,
) {
    suspend fun pullCatalog(nowEpochMs: Long = currentEpochMs()) {
        val commerces = fetchAllCommerces()
        catalog.replaceCommerces(commerces)
        val products = fetchAllProducts()
        catalog.replaceProducts(products.filter { it.active })
        catalog.setLastCatalogSyncEpochMs(nowEpochMs)
    }

    private suspend fun fetchAllCommerces(): List<Commerce> {
        val all = mutableListOf<Commerce>()
        var cursor: String? = null
        while (true) {
            val page = client.fetchCommerces(pageSize, cursor)
            if (page.data.isEmpty()) break
            all += page.data
            if (!page.pagination.hasMore || page.pagination.nextCursor == null) break
            cursor = page.pagination.nextCursor
            if (page.data.size < pageSize) break
        }
        return all
    }

    private suspend fun fetchAllProducts(): List<Product> {
        val all = mutableListOf<Product>()
        var cursor: String? = null
        while (true) {
            val page = client.fetchProducts(pageSize, cursor)
            if (page.data.isEmpty()) break
            all += page.data
            if (!page.pagination.hasMore || page.pagination.nextCursor == null) break
            cursor = page.pagination.nextCursor
            if (page.data.size < pageSize) break
        }
        return all
    }
}

class SellerSyncCoordinator(
    private val catalogPull: CatalogPullSync,
    private val engine: SyncEngine,
) {
    /** Push outbox only — never blocked by catalog pull. */
    suspend fun pushOutbox(): SyncProcessResult = engine.processOutbox()

    /** Best-effort catalog pull; failures are swallowed so push can still succeed. */
    suspend fun pullCatalog(): Boolean =
        runCatching { catalogPull.pullCatalog() }.isSuccess

    /** Push first, then best-effort pull. Catalog failure does not abort push. */
    suspend fun syncPullAndPush(): SyncProcessResult {
        val pushResult = pushOutbox()
        pullCatalog()
        return pushResult
    }
}
