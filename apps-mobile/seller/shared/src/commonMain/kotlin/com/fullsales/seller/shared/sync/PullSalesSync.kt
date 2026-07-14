package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.CursorListSales
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.SaleRepository

interface SalesPullClient {
    suspend fun fetchSales(limit: Int, cursor: String?): CursorListSales
}

/**
 * Pull seller sales into LocalStore (OD-16-7: page until has_more false; max page cap).
 */
class PullSalesSync(
    private val sales: SaleRepository,
    private val client: SalesPullClient,
    private val pageSize: Int = 50,
    private val maxPages: Int = 50,
) {
    suspend fun pullSales(nowEpochMs: Long = currentEpochMs()) {
        sales.upsertFromRemoteSales(fetchAllSales())
        sales.setLastSalesSyncEpochMs(nowEpochMs)
    }

    private suspend fun fetchAllSales(): List<Sale> {
        val all = mutableListOf<Sale>()
        var cursor: String? = null
        var pages = 0
        while (pages < maxPages) {
            pages++
            val page = client.fetchSales(pageSize, cursor)
            if (page.data.isEmpty()) break
            all += page.data
            if (!page.pagination.hasMore || page.pagination.nextCursor == null) break
            cursor = page.pagination.nextCursor
            if (page.data.size < pageSize) break
        }
        return all
    }
}
