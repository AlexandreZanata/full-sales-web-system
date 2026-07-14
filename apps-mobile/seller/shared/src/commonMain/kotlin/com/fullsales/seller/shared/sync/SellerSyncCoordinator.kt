package com.fullsales.seller.shared.sync

/**
 * Push outbox first; catalog/sales/registrations/settings pulls are best-effort and never abort push.
 */
class SellerSyncCoordinator(
    private val catalogPull: CatalogPullSync,
    private val salesPull: PullSalesSync,
    private val registrationsPull: PullRegistrationsSync,
    private val settingsPull: PullSettingsSync?,
    private val engine: SyncEngine,
) {
    suspend fun pushOutbox(): SyncProcessResult = engine.processOutbox()

    suspend fun pullCatalog(): Boolean =
        runCatching { catalogPull.pullCatalog() }.isSuccess

    suspend fun pullSales(): Boolean =
        runCatching { salesPull.pullSales() }.isSuccess

    suspend fun pullRegistrations(): Boolean =
        runCatching { registrationsPull.pullRegistrations() }.isSuccess

    suspend fun pullSettings(): Boolean =
        settingsPull?.let { runCatching { it.pullSettings() }.isSuccess } ?: true

    suspend fun syncPullAndPush(): SyncProcessResult {
        val pushResult = pushOutbox()
        pullCatalog()
        pullSales()
        pullRegistrations()
        pullSettings()
        return pushResult
    }

    /**
     * Same as [syncPullAndPush] but exposes per-domain pull success for keep-cache snackbars (16F).
     */
    suspend fun syncPullAndPushWithPullFlags(): SyncPullFlags {
        pushOutbox()
        return SyncPullFlags(
            catalogOk = pullCatalog(),
            salesOk = pullSales(),
            registrationsOk = pullRegistrations(),
            settingsOk = pullSettings(),
        )
    }
}

data class SyncPullFlags(
    val catalogOk: Boolean,
    val salesOk: Boolean,
    val registrationsOk: Boolean,
    val settingsOk: Boolean,
)
