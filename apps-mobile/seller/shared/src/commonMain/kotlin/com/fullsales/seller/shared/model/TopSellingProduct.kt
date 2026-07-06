package com.fullsales.seller.shared.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class TopSellingProduct(
    @SerialName("productId") val productId: String,
    val name: String,
    val sku: String,
    @SerialName("unitsSold") val unitsSold: Long,
)

@Serializable
data class TopSellingProductsResponse(
    val data: List<TopSellingProduct>,
    val pagination: com.fullsales.seller.shared.model.CursorPaginationMeta,
)
