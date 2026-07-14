package com.fullsales.seller.shared.sync

/**
 * Push outbox first; catalog/sales/registrations pulls are best-effort and never abort push.
 */
class SellerSyncCoordinator(
    private val catalogPull: CatalogPullSync,
    private val salesPull: PullSalesSync,
    private val registrationsPull: PullRegistrationsSync,
    private val engine: SyncEngine,
) {
    suspend fun pushOutbox(): SyncProcessResult = engine.processOutbox()

    suspend fun pullCatalog(): Boolean =
        runCatching { catalogPull.pullCatalog() }.isSuccess

    suspend fun pullSales(): Boolean =
        runCatching { salesPull.pullSales() }.isSuccess

    suspend fun pullRegistrations(): Boolean =
        runCatching { registrationsPull.pullRegistrations() }.isSuccess

    suspend fun syncPullAndPush(): SyncProcessResult {
        val pushResult = pushOutbox()
        pullCatalog()
        pullSales()
        pullRegistrations()
        return pushResult
    }
}
