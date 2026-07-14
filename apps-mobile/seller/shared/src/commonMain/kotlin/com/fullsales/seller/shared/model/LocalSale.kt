package com.fullsales.seller.shared.model

enum class LocalSaleStatus {
    Draft,
    PendingSync,
    Synced,
    SyncFailed,
    Confirmed,
    Cancelled,
}

/** Origin of a LocalSale row in LocalStore (Phase 16A). */
enum class SaleOrigin {
    Local,
    RemoteMirror,
}

data class LocalSale(
    val localId: String,
    val remoteId: String? = null,
    val idempotencyKey: String,
    val commerceId: String,
    val paymentMethod: String,
    val status: LocalSaleStatus,
    val totalAmount: Double,
    val totalCurrency: String = "BRL",
    val items: List<SaleItem> = emptyList(),
    val createdAtEpochMs: Long,
    val syncFailureReason: String? = null,
    val driverId: String? = null,
    val origin: SaleOrigin = SaleOrigin.Local,
)
