package com.fullsales.seller.shared.model

data class SyncOutboxEntry(
    val id: String,
    val saleLocalId: String,
    val method: String,
    val path: String,
    val bodyJson: String,
    val idempotencyKey: String,
    val createdAtEpochMs: Long,
    val attempts: Int = 0,
    val lastError: String? = null,
    val completed: Boolean = false,
)
