package com.fullsales.seller.android.media

import com.fullsales.seller.shared.api.SellerApiClient
import java.time.Instant

class MediaUrlCache(
    private val apiClient: SellerApiClient,
    private val expiryBufferSeconds: Long = 60,
    private val nowEpochSeconds: () -> Long = { Instant.now().epochSecond },
) {
    private data class Cached(val url: String, val expiresAtEpochSeconds: Long)

    private val cache = mutableMapOf<String, Cached>()

    suspend fun resolveImageUrl(directUrl: String?, fileId: String?): String? {
        directUrl?.takeIf { it.isNotBlank() }?.let { return it }
        val id = fileId?.takeIf { it.isNotBlank() } ?: return null
        cache[id]?.takeIf { !isExpired(it) }?.let { return it.url }
        val response = runCatching { apiClient.getMediaUrl(id) }.getOrNull() ?: return null
        val expiresAt = parseEpochSeconds(response.expiresAt) ?: return response.url
        cache[id] = Cached(response.url, expiresAt)
        return response.url
    }

    private fun isExpired(cached: Cached): Boolean =
        nowEpochSeconds() >= cached.expiresAtEpochSeconds - expiryBufferSeconds

    private fun parseEpochSeconds(iso: String): Long? = runCatching {
        Instant.parse(iso).epochSecond
    }.getOrNull()
}
