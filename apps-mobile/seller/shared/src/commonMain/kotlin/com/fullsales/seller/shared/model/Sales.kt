package com.fullsales.seller.shared.model

import kotlinx.serialization.Serializable

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
    val id: String,
    val commerceId: String,
    val driverId: String,
    val status: String,
    val paymentMethod: String,
    val totalAmount: Double,
    val totalCurrency: String,
    val items: List<SaleItem> = emptyList(),
    val createdAt: String? = null,
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

@Serializable
data class PaginatedSales(
    val page: Int,
    val pageSize: Int,
    val total: Int,
    val items: List<Sale>,
)
