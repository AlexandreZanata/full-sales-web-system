package com.fullsales.seller.shared.media

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlinx.coroutines.test.runTest

/**
 * T-16-11 / T-16-12 — durable media URL cache with expiry (OD-16-8 = URL only).
 */
class MediaUrlCacheTest {
    @Test
    fun given_cachedUrlNotExpired_when_offlineResolve_then_returnsCached() = runTest {
        val store = MemoryMediaUrlCacheStore()
        store.put(MediaUrlCacheEntry("f1", "https://cdn/a.jpg", expiresAtEpochMs = 200_000L))
        var fetchCount = 0
        val resolver = MediaUrlCacheResolver(
            store = store,
            fetch = {
                fetchCount++
                null
            },
            nowEpochMs = { 100_000L },
            expiryBufferMs = 60_000L,
        )
        assertEquals("https://cdn/a.jpg", resolver.resolveByFileId("f1"))
        assertEquals(0, fetchCount)
    }

    @Test
    fun given_expiredCachedUrl_when_offlineFetchFails_then_returnsNull() = runTest {
        val store = MemoryMediaUrlCacheStore()
        store.put(MediaUrlCacheEntry("f2", "https://cdn/old.jpg", expiresAtEpochMs = 100_000L))
        val resolver = MediaUrlCacheResolver(
            store = store,
            fetch = { null },
            nowEpochMs = { 100_000L },
            expiryBufferMs = 60_000L,
        )
        assertNull(resolver.resolveByFileId("f2"))
    }

    @Test
    fun given_miss_when_fetchSucceeds_then_persistsAndReturns() = runTest {
        val store = MemoryMediaUrlCacheStore()
        val resolver = MediaUrlCacheResolver(
            store = store,
            fetch = { id -> MediaUrlCacheEntry(id, "https://cdn/new.jpg", 99_000L) },
            nowEpochMs = { 1_000L },
        )
        assertEquals("https://cdn/new.jpg", resolver.resolveByFileId("f3"))
        assertEquals("https://cdn/new.jpg", store.get("f3")?.url)
        assertEquals(99_000L, store.get("f3")?.expiresAtEpochMs)
    }
}
