package com.fullsales.seller.shared.db.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.fullsales.seller.shared.db.entity.MediaUrlCacheEntity
import com.fullsales.seller.shared.db.entity.SiteSettingsEntity

@Dao
interface MediaSettingsDao {
    @Query("SELECT * FROM media_url_cache WHERE fileId = :fileId LIMIT 1")
    suspend fun getMediaUrl(fileId: String): MediaUrlCacheEntity?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertMediaUrl(entry: MediaUrlCacheEntity)

    @Query("SELECT * FROM site_settings WHERE id = :id LIMIT 1")
    suspend fun getSettings(id: String = SiteSettingsEntity.SINGLETON_ID): SiteSettingsEntity?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertSettings(entry: SiteSettingsEntity)
}
