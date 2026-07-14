package com.fullsales.seller.shared.ui

/**
 * Why a LocalStore-backed list has no rows (or why refresh failed while keeping cache).
 * Never invent an “empty list” when cache rows exist — use [RefreshFailedKeepCache] snackbar instead.
 */
enum class ListEmptyReason {
    /** DB empty and this domain has never completed a successful pull. */
    NeverSynced,

    /** At least one successful pull; LocalStore truly has zero rows. */
    SyncedEmpty,

    /** Offline and no local cache — virgin install bootstrap (must ask to connect once). */
    OfflineUnavailable,

    /** Online refresh failed but LocalStore still has rows — keep list, show snackbar. */
    RefreshFailedKeepCache,
}

/**
 * Resolve list UX reason from LocalStore + connectivity.
 *
 * @param hasLocalRows true when LocalStore has ≥1 row (ignore search filters).
 * @param everSynced true when durable sync metadata exists for this domain.
 * @param isOnline validated Online connectivity.
 * @param refreshFailed true when the latest online pull/refresh failed.
 */
fun resolveListEmptyReason(
    hasLocalRows: Boolean,
    everSynced: Boolean,
    isOnline: Boolean,
    refreshFailed: Boolean,
): ListEmptyReason? {
    if (hasLocalRows) {
        return if (refreshFailed) ListEmptyReason.RefreshFailedKeepCache else null
    }
    return when {
        !everSynced && !isOnline -> ListEmptyReason.OfflineUnavailable
        !everSynced -> ListEmptyReason.NeverSynced
        else -> ListEmptyReason.SyncedEmpty
    }
}
