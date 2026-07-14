package com.fullsales.seller.shared.media

import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.repository.SiteSettingsRepository
import com.fullsales.seller.shared.repository.SiteSettingsSnapshot
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class MemoryMediaUrlCacheStore : MediaUrlCacheStore {
    private val mutex = Mutex()
    private val rows = linkedMapOf<String, MediaUrlCacheEntry>()

    override suspend fun get(fileId: String): MediaUrlCacheEntry? = mutex.withLock { rows[fileId] }

    override suspend fun put(entry: MediaUrlCacheEntry) {
        mutex.withLock { rows[entry.fileId] = entry }
    }
}

class MemorySiteSettingsRepository : SiteSettingsRepository {
    private val mutex = Mutex()
    private var snapshot: SiteSettingsSnapshot? = null

    override suspend fun get(): SiteSettingsSnapshot? = mutex.withLock { snapshot }

    override suspend fun upsert(settings: SiteSettings, syncedAtEpochMs: Long) {
        mutex.withLock {
            snapshot = SiteSettingsSnapshot(settings, syncedAtEpochMs)
        }
    }
}
