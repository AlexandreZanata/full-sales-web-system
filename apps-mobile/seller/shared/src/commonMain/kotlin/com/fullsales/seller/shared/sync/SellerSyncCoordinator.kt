package com.fullsales.seller.shared.sync

/**
 * Push outbox first; catalog + sales pulls are best-effort and never abort push.
 */
class SellerSyncCoordinator(
    private val catalogPull: CatalogPullSync,
    private val salesPull: PullSalesSync,
    private val engine: SyncEngine,
) {
    suspend fun pushOutbox(): SyncProcessResult = engine.processOutbox()

    suspend fun pullCatalog(): Boolean =
        runCatching { catalogPull.pullCatalog() }.isSuccess

    suspend fun pullSales(): Boolean =
        runCatching { salesPull.pullSales() }.isSuccess

    suspend fun syncPullAndPush(): SyncProcessResult {
        val pushResult = pushOutbox()
        pullCatalog()
        pullSales()
        return pushResult
    }
}
