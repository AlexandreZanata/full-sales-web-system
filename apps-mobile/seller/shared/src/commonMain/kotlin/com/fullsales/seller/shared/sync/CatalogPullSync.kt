package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CursorListProducts
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.CatalogRepository

interface CatalogPullClient {
    suspend fun fetchCommerces(page: Int, pageSize: Int): List<Commerce>
    suspend fun fetchProducts(limit: Int, cursor: String?): CursorListProducts
}

class CatalogPullSync(
    private val catalog: CatalogRepository,
    private val client: CatalogPullClient,
    private val pageSize: Int = 50,
) {
    suspend fun pullCatalog(nowEpochMs: Long = currentEpochMs()) {
        val commerces = fetchAllPages { page -> client.fetchCommerces(page, pageSize) }
        catalog.replaceCommerces(commerces)
        val products = fetchAllProducts()
        catalog.replaceProducts(products.filter { it.active })
        catalog.setLastCatalogSyncEpochMs(nowEpochMs)
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

class SellerSyncCoordinator(
    private val catalogPull: CatalogPullSync,
    private val engine: SyncEngine,
) {
    suspend fun syncPullAndPush(): SyncProcessResult {
        catalogPull.pullCatalog()
        return engine.processOutbox()
    }
}
