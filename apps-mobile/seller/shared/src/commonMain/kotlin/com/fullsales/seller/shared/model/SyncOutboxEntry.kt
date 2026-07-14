package com.fullsales.seller.shared.model

object SyncEntityType {
    const val Sale = "Sale"
    const val Registration = "Registration"
}

data class SyncOutboxEntry(
    val id: String,
    /** Aggregate id (sale localId or registration localId). */
    val saleLocalId: String,
    val method: String,
    val path: String,
    val bodyJson: String,
    val idempotencyKey: String,
    val createdAtEpochMs: Long,
    val attempts: Int = 0,
    val lastError: String? = null,
    val completed: Boolean = false,
    val entityType: String = SyncEntityType.Sale,
)
