package com.fullsales.seller.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class Commerce(
    val id: String,
    val legalName: String,
    val tradeName: String? = null,
    val active: Boolean,
)

@Serializable
data class CommerceAddress(
    val id: String,
    val type: String,
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
