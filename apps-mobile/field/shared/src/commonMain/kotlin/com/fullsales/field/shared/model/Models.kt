package com.fullsales.field.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class Commerce(
    val id: String,
    val legalName: String,
    val tradeName: String? = null,
    val active: Boolean,
)

@Serializable
data class Product(
    val id: String,
    val name: String,
    val sku: String,
    val priceAmount: Double,
    val priceCurrency: String,
    val active: Boolean,
)

@Serializable
data class StockBalance(
    val productId: String,
    val available: Int,
    val asOf: String,
)

@Serializable
data class SaleItem(
    val productId: String,
    val quantity: Int,
    val unitPriceAmount: Double = 0.0,
    val unitPriceCurrency: String = "BRL",
    val lineTotalAmount: Double = 0.0,
)

@Serializable
data class Sale(
    val localId: String,
    val remoteId: String? = null,
    val commerceId: String,
    val driverId: String = "",
    val status: LocalSaleStatus,
    val paymentMethod: String,
    val totalAmount: Double,
    val totalCurrency: String,
    val items: List<SaleItem> = emptyList(),
    val createdAtEpochMs: Long = 0L,
)

@Serializable
data class CreateSaleRequest(
    val commerceId: String,
    val items: List<CreateSaleItem>,
    val paymentMethod: String,
)

@Serializable
data class CreateSaleItem(
    val productId: String,
    val quantity: Int,
)

enum class LocalSaleStatus {
    DraftLocal,
    PendingSync,
    PendingRemote,
    Confirmed,
    Cancelled,
    SyncFailed,
}
