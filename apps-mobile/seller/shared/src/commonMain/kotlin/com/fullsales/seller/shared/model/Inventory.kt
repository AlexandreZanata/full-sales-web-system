package com.fullsales.seller.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class StockBalance(
    val productId: String,
    val available: Int,
    val asOf: String? = null,
)
