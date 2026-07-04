package com.fullsales.field.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class PaginatedCommerces(
    val page: Int,
    val pageSize: Int,
    val total: Int,
    val items: List<Commerce>,
)

@Serializable
data class PaginatedProducts(
    val page: Int,
    val pageSize: Int,
    val total: Int,
    val items: List<Product>,
)

@Serializable
data class PaginatedSales(
    val page: Int,
    val pageSize: Int,
    val total: Int,
    val items: List<SaleDto>,
)

@Serializable
data class SaleDto(
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
