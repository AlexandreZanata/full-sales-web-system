package com.fullsales.seller.shared.db

import androidx.room.migration.Migration
import androidx.sqlite.db.SupportSQLiteDatabase

/** Room v7→v8: durable media URL cache + site settings snapshot (16E). */
object SellerMigrationsV8 {
    val MIGRATION_7_8: Migration = object : Migration(7, 8) {
        override fun migrate(db: SupportSQLiteDatabase) {
            db.execSQL(
                """
                CREATE TABLE IF NOT EXISTS media_url_cache (
                  fileId TEXT NOT NULL PRIMARY KEY,
                  url TEXT NOT NULL,
                  expiresAtEpochMs INTEGER NOT NULL
                )
                """.trimIndent(),
            )
            db.execSQL(
                """
                CREATE TABLE IF NOT EXISTS site_settings (
                  id TEXT NOT NULL PRIMARY KEY,
                  displayName TEXT NOT NULL,
                  logoFileId TEXT,
                  logoUrl TEXT,
                  salesContactPhone TEXT,
                  syncedAtEpochMs INTEGER NOT NULL
                )
                """.trimIndent(),
            )
        }
    }
}
