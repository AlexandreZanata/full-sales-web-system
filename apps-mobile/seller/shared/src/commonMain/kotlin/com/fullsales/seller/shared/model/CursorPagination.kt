package com.fullsales.seller.shared.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class CursorPaginationMeta(
    @SerialName("next_cursor") val nextCursor: String? = null,
    @SerialName("has_more") val hasMore: Boolean,
    val limit: Int,
)

@Serializable
data class CursorListProducts(
    val data: List<Product>,
    val pagination: CursorPaginationMeta,
)

/** Legacy offset shape — commerces migrate in phase 68C */
@Serializable
data class PaginatedProducts(
    val page: Int,
    val pageSize: Int,
    val total: Int,
    val items: List<Product>,
)
