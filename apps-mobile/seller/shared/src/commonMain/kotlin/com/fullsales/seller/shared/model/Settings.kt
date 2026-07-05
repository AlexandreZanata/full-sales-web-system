package com.fullsales.seller.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class SiteSettings(
    val displayName: String,
    val logoFileId: String? = null,
    val logoUrl: String? = null,
    val salesContactPhone: String? = null,
)
