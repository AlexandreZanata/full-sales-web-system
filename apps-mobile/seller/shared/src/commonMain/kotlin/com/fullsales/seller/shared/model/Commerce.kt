package com.fullsales.seller.shared.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class Commerce(
    val id: String,
    val legalName: String,
    val tradeName: String? = null,
    val active: Boolean,
    val cnpj: String? = null,
)

@Serializable
data class CommerceAddress(
    val id: String,
    @SerialName("addressType") val type: String,
    val street: String,
    val number: String,
    val city: String,
    val state: String,
    val postalCode: String,
    val isPrimary: Boolean = false,
)

@Serializable
data class PaginatedCommerces(
    val page: Int,
    val pageSize: Int,
    val total: Int,
    val items: List<Commerce>,
)
