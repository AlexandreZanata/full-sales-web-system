package com.fullsales.seller.shared.model

enum class SaleDisplayStatus {
    Pending,
    Confirmed,
    Cancelled,
    PendingSync,
    SyncFailed,
}

data class SalesListItem(
    val navigationId: String,
    val shortId: String,
    val createdAtEpochMs: Long,
    val status: SaleDisplayStatus,
    val totalAmount: Double,
    val totalCurrency: String,
)
