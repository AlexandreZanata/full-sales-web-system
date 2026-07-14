package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.SiteSettings

data class SiteSettingsSnapshot(
    val settings: SiteSettings,
    val syncedAtEpochMs: Long,
)

interface SiteSettingsRepository {
    suspend fun get(): SiteSettingsSnapshot?
    suspend fun upsert(settings: SiteSettings, syncedAtEpochMs: Long)
}
