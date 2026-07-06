package com.fullsales.field.shared.sync

import com.fullsales.field.shared.model.CursorListCommerces
import com.fullsales.field.shared.repository.CatalogRepository

interface CatalogPullClient {
    suspend fun fetchCommerces(limit: Int, cursor: String?): CursorListCommerces
    suspend fun fetchProducts(page: Int, pageSize: Int): List<com.fullsales.field.shared.model.Product>
    suspend fun fetchStockBalance(productId: String): com.fullsales.field.shared.model.StockBalance?
}

class CatalogPullSync(
    private val catalog: CatalogRepository,
    private val client: CatalogPullClient,
    private val pageSize: Int = 50,
) {
    suspend fun pullCatalogAndStock(nowEpochMs: Long = System.currentTimeMillis()) {
        val commerces = fetchAllCommerces()
        catalog.replaceCommerces(commerces)
        val products = fetchAllPages { page -> client.fetchProducts(page, pageSize) }
        catalog.replaceProducts(products.filter { it.active })
        for (productId in catalog.listProductIds()) {
            client.fetchStockBalance(productId)?.let { catalog.upsertStockBalance(it) }
        }
        catalog.setLastCatalogSyncEpochMs(nowEpochMs)
    }

    private suspend fun fetchAllCommerces(): List<com.fullsales.field.shared.model.Commerce> {
        val all = mutableListOf<com.fullsales.field.shared.model.Commerce>()
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

    private suspend fun <T> fetchAllPages(fetch: suspend (Int) -> List<T>): List<T> {
        val all = mutableListOf<T>()
        var page = 1
        while (true) {
            val batch = fetch(page)
            if (batch.isEmpty()) break
            all += batch
            if (batch.size < pageSize) break
            page++
        }
        return all
    }
}

class FieldSyncCoordinator(
    private val catalogPull: CatalogPullSync,
    private val engine: SyncEngine,
) {
    suspend fun syncPullAndPush(): SyncProcessResult {
        catalogPull.pullCatalogAndStock()
        return engine.processOutbox()
    }
}
