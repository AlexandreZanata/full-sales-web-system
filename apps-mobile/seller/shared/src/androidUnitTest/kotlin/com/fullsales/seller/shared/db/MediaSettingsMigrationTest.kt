package com.fullsales.seller.shared.db

import android.content.Context
import androidx.sqlite.db.framework.FrameworkSQLiteOpenHelperFactory
import androidx.test.core.app.ApplicationProvider
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

/** Phase 16E — Migration v7→v8 creates media_url_cache + site_settings. */
@RunWith(RobolectricTestRunner::class)
class MediaSettingsMigrationTest {
    @Test
    fun given_v7Db_when_migrateToV8_then_mediaAndSettingsTablesExist() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val dbName = "seller-migration-7-8.db"
        context.deleteDatabase(dbName)

        val helper = FrameworkSQLiteOpenHelperFactory().create(
            androidx.sqlite.db.SupportSQLiteOpenHelper.Configuration.builder(context)
                .name(dbName)
                .callback(
                    object : androidx.sqlite.db.SupportSQLiteOpenHelper.Callback(7) {
                        override fun onCreate(db: androidx.sqlite.db.SupportSQLiteDatabase) {
                            SellerMigrations.createV6CoreTables(db)
                            SellerMigrations.MIGRATION_6_7.migrate(db)
                        }

                        override fun onUpgrade(
                            db: androidx.sqlite.db.SupportSQLiteDatabase,
                            oldVersion: Int,
                            newVersion: Int,
                        ) = Unit
                    },
                )
                .build(),
        )

        helper.writableDatabase.use { db ->
            SellerMigrationsV8.MIGRATION_7_8.migrate(db)
            db.query(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='media_url_cache'",
            ).use { cursor ->
                assertTrue(cursor.moveToFirst())
                assertEquals("media_url_cache", cursor.getString(0))
            }
            db.query(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='site_settings'",
            ).use { cursor ->
                assertTrue(cursor.moveToFirst())
                assertEquals("site_settings", cursor.getString(0))
            }
        }
        context.deleteDatabase(dbName)
    }
}
