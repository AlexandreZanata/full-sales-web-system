package com.fullsales.seller.shared.model

import kotlinx.serialization.Serializable

@Serializable
data class MediaUrlResponse(
    val url: String,
    val expiresAt: String,
)

@Serializable
data class MediaUploadResponse(
    val id: String,
    val entityType: String,
    val entityId: String,
    val mimeType: String,
    val sizeBytes: Long,
    val sha256: String,
)
