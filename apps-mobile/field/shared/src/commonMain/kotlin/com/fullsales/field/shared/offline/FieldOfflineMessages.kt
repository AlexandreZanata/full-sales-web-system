package com.fullsales.field.shared.offline

/** Copy helpers for Field offline empty states (Phase 18F). */
object FieldOfflineMessages {
    fun salesEmpty(online: Boolean): String =
        if (online) "No sales yet" else "Offline — sync when online"

    fun catalogEmptyOffline(): String =
        "Offline — connect once to download catalog"

    fun bannerTitle(): String = "You're offline"
}
