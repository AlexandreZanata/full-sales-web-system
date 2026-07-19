package com.fullsales.seller.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class SellerShare(
    val publicCode: String,
    val sharePath: String,
    /** Absolute catalog URL from API (`PORTAL_PUBLIC_ORIGIN` + path). */
    val shareUrl: String,
    val contactPhone: String? = null,
    val shareLinkActive: Boolean = true,
)
