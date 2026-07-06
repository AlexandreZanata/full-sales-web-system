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

@Serializable
data class CursorListCommerces(
    val data: List<Commerce>,
    val pagination: CursorPaginationMeta,
)

@Serializable
data class CursorListCommerceAddresses(
    val data: List<CommerceAddress>,
    val pagination: CursorPaginationMeta,
)

@Serializable
data class CursorListRegistrations(
    val data: List<CommerceRegistration>,
    val pagination: CursorPaginationMeta,
)
