package com.fullsales.seller.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class SellerProfile(
    val userId: String,
    val operatingRegion: String? = null,
    val monthlyTargetAmount: Long? = null,
    val publicCode: String? = null,
    val contactPhone: String? = null,
    val shareLinkActive: Boolean = true,
)

@Serializable
data class PatchSellerProfileRequest(
    val operatingRegion: String? = null,
    val contactPhone: String? = null,
    val shareLinkActive: Boolean? = null,
)
