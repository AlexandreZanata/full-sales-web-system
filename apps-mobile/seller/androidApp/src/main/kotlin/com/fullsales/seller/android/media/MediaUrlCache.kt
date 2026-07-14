package com.fullsales.seller.android.media

import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.apiBaseUrl
import com.fullsales.seller.shared.media.MediaUrlCacheEntry
import com.fullsales.seller.shared.media.MediaUrlCacheResolver
import com.fullsales.seller.shared.media.MediaUrlCacheStore
import com.fullsales.seller.shared.media.parseMediaExpiresAtEpochMs
import com.fullsales.seller.shared.media.productThumbnailLoadUrl
import com.fullsales.seller.shared.model.currentEpochMs

class MediaUrlCache(
    private val apiClient: SellerApiClient,
    private val store: MediaUrlCacheStore,
    expiryBufferMs: Long = 60_000L,
    nowEpochMs: () -> Long = { currentEpochMs() },
) : MediaUrlResolver {
    private val resolver = MediaUrlCacheResolver(
        store = store,
        fetch = { fileId -> fetchAndNormalize(fileId) },
        nowEpochMs = nowEpochMs,
        expiryBufferMs = expiryBufferMs,
    )

    override suspend fun resolveImageUrl(directUrl: String?, fileId: String?): String? {
        directUrl?.takeIf { it.isNotBlank() }?.let {
            return productThumbnailLoadUrl(it, apiBaseUrl)
        }
        val id = fileId?.takeIf { it.isNotBlank() } ?: return null
        return resolver.resolveByFileId(id)
    }

    private suspend fun fetchAndNormalize(fileId: String): MediaUrlCacheEntry? {
        val response = runCatching { apiClient.getMediaUrl(fileId) }.getOrNull() ?: return null
        val loadable = productThumbnailLoadUrl(response.url, apiBaseUrl)
        val expiresAt = parseMediaExpiresAtEpochMs(response.expiresAt) ?: return null
        return MediaUrlCacheEntry(fileId, loadable, expiresAt)
    }
}
