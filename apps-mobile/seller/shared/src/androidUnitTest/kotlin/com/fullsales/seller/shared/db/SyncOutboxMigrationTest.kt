package com.fullsales.seller.shared.db

import android.content.Context
import androidx.sqlite.db.framework.FrameworkSQLiteOpenHelperFactory
import androidx.test.core.app.ApplicationProvider
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

/**
 * Phase 16D — Migration v6→v7 renames outbox aggregate key + adds dependsOnOutboxId.
 */
@RunWith(RobolectricTestRunner::class)
class SyncOutboxMigrationTest {
    @Test
    fun given_v6OutboxRow_when_migrateToV7_then_aggregateIdAndDependsOnExist() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val dbName = "seller-migration-6-7.db"
        context.deleteDatabase(dbName)

        val helper = FrameworkSQLiteOpenHelperFactory().create(
            androidx.sqlite.db.SupportSQLiteOpenHelper.Configuration.builder(context)
                .name(dbName)
                .callback(
                    object : androidx.sqlite.db.SupportSQLiteOpenHelper.Callback(6) {
                        override fun onCreate(db: androidx.sqlite.db.SupportSQLiteDatabase) {
                            SellerMigrations.createV6CoreTables(db)
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
            db.execSQL(
                """
                INSERT INTO sync_outbox (
                  id, saleLocalId, method, path, bodyJson, idempotencyKey,
                  createdAtEpochMs, attempts, lastError, completed, entityType
                ) VALUES (
                  'o2', 'sale-2', 'POST', '/sales', '{}', 'idem-2',
                  2000, 1, NULL, 0, 'Sale'
                )
                """.trimIndent(),
            )
            SellerMigrations.MIGRATION_6_7.migrate(db)

            db.query(
                "SELECT id, aggregateId, entityType, dependsOnOutboxId FROM sync_outbox",
            ).use { cursor ->
                assertTrue(cursor.moveToFirst())
                assertEquals("o2", cursor.getString(0))
                assertEquals("sale-2", cursor.getString(1))
                assertEquals("Sale", cursor.getString(2))
                assertTrue(cursor.isNull(3))
            }
        }
        context.deleteDatabase(dbName)
    }
}
