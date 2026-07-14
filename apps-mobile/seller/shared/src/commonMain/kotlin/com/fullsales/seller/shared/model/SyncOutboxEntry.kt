package com.fullsales.seller.shared.model

object SyncEntityType {
    const val Sale = "Sale"
    const val Registration = "Registration"
}

/**
 * Durable sync queue row (Phase 16D).
 * [aggregateId] is the local sale or registration id; [dependsOnOutboxId] chains confirm/cancel after create.
 */
data class SyncOutboxEntry(
    val id: String,
    val aggregateId: String,
    val method: String,
    val path: String,
    val bodyJson: String,
    val idempotencyKey: String,
    val createdAtEpochMs: Long,
    val attempts: Int = 0,
    val lastError: String? = null,
    val completed: Boolean = false,
    val entityType: String = SyncEntityType.Sale,
    val dependsOnOutboxId: String? = null,
)
