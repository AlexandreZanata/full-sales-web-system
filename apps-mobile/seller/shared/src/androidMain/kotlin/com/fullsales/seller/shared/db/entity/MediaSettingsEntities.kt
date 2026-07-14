package com.fullsales.seller.shared.db.entity

import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "media_url_cache")
data class MediaUrlCacheEntity(
    @PrimaryKey val fileId: String,
    val url: String,
    val expiresAtEpochMs: Long,
)

@Entity(tableName = "site_settings")
data class SiteSettingsEntity(
    @PrimaryKey val id: String = SINGLETON_ID,
    val displayName: String,
    val logoFileId: String?,
    val logoUrl: String?,
    val salesContactPhone: String?,
    val syncedAtEpochMs: Long,
) {
    companion object {
        const val SINGLETON_ID = "default"
    }
}
