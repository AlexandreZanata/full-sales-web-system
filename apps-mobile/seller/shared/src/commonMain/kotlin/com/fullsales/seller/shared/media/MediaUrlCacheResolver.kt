package com.fullsales.seller.shared.media

import com.fullsales.seller.shared.model.currentEpochMs
import kotlinx.datetime.Instant

data class MediaUrlCacheEntry(
    val fileId: String,
    val url: String,
    val expiresAtEpochMs: Long,
)

interface MediaUrlCacheStore {
    suspend fun get(fileId: String): MediaUrlCacheEntry?
    suspend fun put(entry: MediaUrlCacheEntry)
}

/**
 * OD-16-8: durable URL cache only — never crash on expired/missing; return null for placeholder.
 */
class MediaUrlCacheResolver(
    private val store: MediaUrlCacheStore,
    private val fetch: suspend (fileId: String) -> MediaUrlCacheEntry?,
    private val nowEpochMs: () -> Long = { currentEpochMs() },
    private val expiryBufferMs: Long = 60_000L,
) {
    suspend fun resolveByFileId(fileId: String): String? {
        store.get(fileId)?.takeIf { !isExpired(it) }?.let { return it.url }
        val fresh = fetch(fileId) ?: return null
        store.put(fresh)
        return fresh.url
    }

    fun isExpired(entry: MediaUrlCacheEntry, now: Long = nowEpochMs()): Boolean =
        now >= entry.expiresAtEpochMs - expiryBufferMs
}

fun parseMediaExpiresAtEpochMs(iso: String): Long? = runCatching {
    Instant.parse(iso).toEpochMilliseconds()
}.getOrNull()
