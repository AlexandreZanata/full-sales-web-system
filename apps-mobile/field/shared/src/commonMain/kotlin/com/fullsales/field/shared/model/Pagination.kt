package com.fullsales.field.shared.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class CursorPaginationMeta(
    @SerialName("next_cursor") val nextCursor: String? = null,
    @SerialName("has_more") val hasMore: Boolean,
    val limit: Int,
)

@Serializable
data class CursorListCommerces(
    val data: List<Commerce>,
    val pagination: CursorPaginationMeta,
)

/** Legacy offset shape — products/sales migrate in later phases */
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
data class CursorListSales(
    val data: List<SaleDto>,
    val pagination: CursorPaginationMeta,
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
