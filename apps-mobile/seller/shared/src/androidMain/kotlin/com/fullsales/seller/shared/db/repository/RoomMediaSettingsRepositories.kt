package com.fullsales.seller.shared.db.repository

import com.fullsales.seller.shared.db.dao.MediaSettingsDao
import com.fullsales.seller.shared.db.entity.MediaUrlCacheEntity
import com.fullsales.seller.shared.db.entity.SiteSettingsEntity
import com.fullsales.seller.shared.media.MediaUrlCacheEntry
import com.fullsales.seller.shared.media.MediaUrlCacheStore
import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.repository.SiteSettingsRepository
import com.fullsales.seller.shared.repository.SiteSettingsSnapshot

class RoomMediaUrlCacheStore(
    private val dao: MediaSettingsDao,
) : MediaUrlCacheStore {
    override suspend fun get(fileId: String): MediaUrlCacheEntry? =
        dao.getMediaUrl(fileId)?.let {
            MediaUrlCacheEntry(it.fileId, it.url, it.expiresAtEpochMs)
        }

    override suspend fun put(entry: MediaUrlCacheEntry) {
        dao.upsertMediaUrl(
            MediaUrlCacheEntity(entry.fileId, entry.url, entry.expiresAtEpochMs),
        )
    }
}

class RoomSiteSettingsRepository(
    private val dao: MediaSettingsDao,
) : SiteSettingsRepository {
    override suspend fun get(): SiteSettingsSnapshot? {
        val row = dao.getSettings() ?: return null
        return SiteSettingsSnapshot(
            settings = SiteSettings(
                displayName = row.displayName,
                logoFileId = row.logoFileId,
                logoUrl = row.logoUrl,
                salesContactPhone = row.salesContactPhone,
            ),
            syncedAtEpochMs = row.syncedAtEpochMs,
        )
    }

    override suspend fun upsert(settings: SiteSettings, syncedAtEpochMs: Long) {
        dao.upsertSettings(
            SiteSettingsEntity(
                displayName = settings.displayName,
                logoFileId = settings.logoFileId,
                logoUrl = settings.logoUrl,
                salesContactPhone = settings.salesContactPhone,
                syncedAtEpochMs = syncedAtEpochMs,
            ),
        )
    }
}
