package com.fullsales.seller.android.media

import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.apiBaseUrl
import com.fullsales.seller.shared.media.productThumbnailLoadUrl
import java.time.Instant

class MediaUrlCache(
    private val apiClient: SellerApiClient,
    private val expiryBufferSeconds: Long = 60,
    private val nowEpochSeconds: () -> Long = { Instant.now().epochSecond },
) : MediaUrlResolver {
    private data class Cached(val url: String, val expiresAtEpochSeconds: Long)

    private val cache = mutableMapOf<String, Cached>()

    override suspend fun resolveImageUrl(directUrl: String?, fileId: String?): String? {
        directUrl?.takeIf { it.isNotBlank() }?.let {
            return productThumbnailLoadUrl(it, apiBaseUrl)
        }
        val id = fileId?.takeIf { it.isNotBlank() } ?: return null
        cache[id]?.takeIf { !isExpired(it) }?.let { return it.url }
        val response = runCatching { apiClient.getMediaUrl(id) }.getOrNull() ?: return null
        val loadable = productThumbnailLoadUrl(response.url, apiBaseUrl)
        val expiresAt = parseEpochSeconds(response.expiresAt) ?: return loadable
        cache[id] = Cached(loadable, expiresAt)
        return loadable
    }

    private fun isExpired(cached: Cached): Boolean =
        nowEpochSeconds() >= cached.expiresAtEpochSeconds - expiryBufferSeconds

    private fun parseEpochSeconds(iso: String): Long? = runCatching {
        Instant.parse(iso).epochSecond
    }.getOrNull()
}
