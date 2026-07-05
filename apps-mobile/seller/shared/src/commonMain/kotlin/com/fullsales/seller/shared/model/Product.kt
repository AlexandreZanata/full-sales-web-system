package com.fullsales.seller.shared.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class Product(
    val id: String,
    val name: String,
    val sku: String,
    val priceAmount: Double,
    val priceCurrency: String,
    val active: Boolean,
    val categoryId: String? = null,
    val categoryName: String? = null,
    val categorySlug: String? = null,
    @SerialName("primaryImageUrl") val primaryImageUrl: String? = null,
    @SerialName("primaryImageFileId") val primaryImageFileId: String? = null,
)

@Serializable
data class ProductDetail(
    val id: String,
    val name: String,
    val sku: String,
    val priceAmount: Double,
    val priceCurrency: String,
    val active: Boolean,
    val categoryId: String? = null,
    val categoryName: String? = null,
    val categorySlug: String? = null,
    val unitOfMeasure: String? = null,
    val description: String? = null,
    @SerialName("primaryImageUrl") val primaryImageUrl: String? = null,
    @SerialName("primaryImageFileId") val primaryImageFileId: String? = null,
)

@Serializable
data class PaginatedProducts(
    val page: Int,
    val pageSize: Int,
    val total: Int,
    val items: List<Product>,
)
