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
 * T-16-21 — Migration v4→v5 keeps existing sales (OD-16-4).
 */
@RunWith(RobolectricTestRunner::class)
class SellerDatabaseMigrationTest {
    @Test
    fun given_v4SaleRow_when_migrateToV5_then_saleSurvivesAndNewColumnsExist() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val dbName = "seller-migration-4-5.db"
        context.deleteDatabase(dbName)

        val helper = FrameworkSQLiteOpenHelperFactory().create(
            androidx.sqlite.db.SupportSQLiteOpenHelper.Configuration.builder(context)
                .name(dbName)
                .callback(
                    object : androidx.sqlite.db.SupportSQLiteOpenHelper.Callback(4) {
                        override fun onCreate(db: androidx.sqlite.db.SupportSQLiteDatabase) {
                            SellerMigrations.createV4CoreTables(db)
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
                INSERT INTO sales (
                  localId, remoteId, idempotencyKey, commerceId, paymentMethod,
                  status, totalAmount, totalCurrency, createdAtEpochMs, syncFailureReason
                ) VALUES (
                  'sale-1', NULL, 'idem-1', 'commerce-1', 'cash',
                  'PendingSync', 42.5, 'BRL', 1000, NULL
                )
                """.trimIndent(),
            )
            db.execSQL(
                """
                INSERT INTO sale_lines (saleLocalId, productId, quantity)
                VALUES ('sale-1', 'product-1', 2)
                """.trimIndent(),
            )
            SellerMigrations.MIGRATION_4_5.migrate(db)

            db.query("SELECT localId, totalAmount, origin, driverId FROM sales").use { cursor ->
                assertTrue(cursor.moveToFirst())
                assertEquals("sale-1", cursor.getString(0))
                assertEquals(42.5, cursor.getDouble(1), 0.001)
                assertEquals("Local", cursor.getString(2))
                assertTrue(cursor.isNull(3))
            }
            db.query(
                "SELECT unitPriceAmount, unitPriceCurrency, lineTotalAmount FROM sale_lines",
            ).use { cursor ->
                assertTrue(cursor.moveToFirst())
                assertEquals(0.0, cursor.getDouble(0), 0.001)
                assertEquals("BRL", cursor.getString(1))
                assertEquals(0.0, cursor.getDouble(2), 0.001)
            }
            db.query("PRAGMA table_info(commerces)").use { cursor ->
                val cols = mutableSetOf<String>()
                while (cursor.moveToNext()) cols += cursor.getString(1)
                assertTrue(cols.contains("cnpj"))
            }
            db.query("PRAGMA table_info(products)").use { cursor ->
                val cols = mutableSetOf<String>()
                while (cursor.moveToNext()) cols += cursor.getString(1)
                assertTrue(cols.contains("unitOfMeasure"))
                assertTrue(cols.contains("description"))
            }
        }
        context.deleteDatabase(dbName)
    }

    @Test
    fun given_v5Outbox_when_migrateToV6_then_registrationsTableAndEntityTypeExist() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val dbName = "seller-migration-5-6.db"
        context.deleteDatabase(dbName)

        val helper = FrameworkSQLiteOpenHelperFactory().create(
            androidx.sqlite.db.SupportSQLiteOpenHelper.Configuration.builder(context)
                .name(dbName)
                .callback(
                    object : androidx.sqlite.db.SupportSQLiteOpenHelper.Callback(5) {
                        override fun onCreate(db: androidx.sqlite.db.SupportSQLiteDatabase) {
                            SellerMigrations.createV5CoreTables(db)
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
                  createdAtEpochMs, attempts, lastError, completed
                ) VALUES (
                  'o1', 'sale-1', 'POST', '/sales', '{}', 'idem-1',
                  1000, 0, NULL, 0
                )
                """.trimIndent(),
            )
            SellerMigrations.MIGRATION_5_6.migrate(db)

            db.query("SELECT id, entityType FROM sync_outbox").use { cursor ->
                assertTrue(cursor.moveToFirst())
                assertEquals("o1", cursor.getString(0))
                assertEquals("Sale", cursor.getString(1))
            }
            db.query(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='registrations'",
            ).use { cursor ->
                assertTrue(cursor.moveToFirst())
                assertEquals("registrations", cursor.getString(0))
            }
        }
        context.deleteDatabase(dbName)
    }
}
