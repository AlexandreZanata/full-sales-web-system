package com.fullsales.seller.shared.offline

import com.fullsales.seller.shared.connectivity.ConnectivityState
import kotlinx.datetime.Instant

/** Why the sticky Offline banner is shown (Phase 18B). */
enum class OfflineBannerReason {
    None,
    Network,
    Server,
}

data class OfflineBannerState(
    val visible: Boolean,
    val reason: OfflineBannerReason,
    val pendingCount: Int,
) {
    val showPendingChip: Boolean get() = visible && pendingCount > 0
}

/**
 * Contract: banner visible when Offline (network) or Online+serverUnreachable;
 * hidden when Online and healthy.
 */
fun resolveOfflineBanner(
    connectivity: ConnectivityState,
    serverUnreachable: Boolean,
    pendingCount: Int,
): OfflineBannerState {
    val reason = when {
        connectivity == ConnectivityState.Offline -> OfflineBannerReason.Network
        connectivity == ConnectivityState.Online && serverUnreachable -> OfflineBannerReason.Server
        else -> OfflineBannerReason.None
    }
    return OfflineBannerState(
        visible = reason != OfflineBannerReason.None,
        reason = reason,
        pendingCount = pendingCount.coerceAtLeast(0),
    )
}

/** Create-sale fields that must remain populated from LocalStore while Offline (18D). */
object OfflineFieldVisibility {
    val createSaleMustShow = listOf(
        "commerces",
        "products",
        "paymentMethod",
        "lines",
        "submit",
    )
    val internetOnlyDisabledWithWarn = listOf(
        "cnpjLookup",
        "mediaUpload",
        "topSellingChips",
    )
}

fun formatSyncEpochIso(epochMs: Long?): String? =
    epochMs?.let { Instant.fromEpochMilliseconds(it).toString() }
