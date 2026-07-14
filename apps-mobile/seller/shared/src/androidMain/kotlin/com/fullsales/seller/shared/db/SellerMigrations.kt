package com.fullsales.seller.shared.db

import androidx.room.migration.Migration
import androidx.sqlite.db.SupportSQLiteDatabase

/**
 * Room migrations for Seller LocalStore.
 * OD-16-4: explicit Migration v4→v5 — do not wipe field data on this bump.
 */
object SellerMigrations {
    val MIGRATION_4_5: Migration = object : Migration(4, 5) {
        override fun migrate(db: SupportSQLiteDatabase) {
            db.execSQL("ALTER TABLE commerces ADD COLUMN cnpj TEXT")
            db.execSQL("ALTER TABLE products ADD COLUMN unitOfMeasure TEXT")
            db.execSQL("ALTER TABLE products ADD COLUMN description TEXT")
            db.execSQL("ALTER TABLE sales ADD COLUMN driverId TEXT")
            db.execSQL(
                "ALTER TABLE sales ADD COLUMN origin TEXT NOT NULL DEFAULT 'Local'",
            )
            db.execSQL(
                "ALTER TABLE sale_lines ADD COLUMN unitPriceAmount REAL NOT NULL DEFAULT 0.0",
            )
            db.execSQL(
                "ALTER TABLE sale_lines ADD COLUMN unitPriceCurrency TEXT NOT NULL DEFAULT 'BRL'",
            )
            db.execSQL(
                "ALTER TABLE sale_lines ADD COLUMN lineTotalAmount REAL NOT NULL DEFAULT 0.0",
            )
        }
    }

    /** Minimal v4 DDL for migration contract tests (tables touched by 4→5). */
    fun createV4CoreTables(db: SupportSQLiteDatabase) {
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS commerces (
              id TEXT NOT NULL PRIMARY KEY,
              legalName TEXT NOT NULL,
              tradeName TEXT,
              active INTEGER NOT NULL
            )
            """.trimIndent(),
        )
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS products (
              id TEXT NOT NULL PRIMARY KEY,
              name TEXT NOT NULL,
              sku TEXT NOT NULL,
              priceAmount REAL NOT NULL,
              priceCurrency TEXT NOT NULL,
              active INTEGER NOT NULL,
              categoryId TEXT,
              categoryName TEXT,
              categorySlug TEXT,
              primaryImageUrl TEXT,
              primaryImageFileId TEXT
            )
            """.trimIndent(),
        )
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS sales (
              localId TEXT NOT NULL PRIMARY KEY,
              remoteId TEXT,
              idempotencyKey TEXT NOT NULL,
              commerceId TEXT NOT NULL,
              paymentMethod TEXT NOT NULL,
              status TEXT NOT NULL,
              totalAmount REAL NOT NULL,
              totalCurrency TEXT NOT NULL,
              createdAtEpochMs INTEGER NOT NULL,
              syncFailureReason TEXT
            )
            """.trimIndent(),
        )
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS sale_lines (
              id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
              saleLocalId TEXT NOT NULL,
              productId TEXT NOT NULL,
              quantity INTEGER NOT NULL,
              FOREIGN KEY(saleLocalId) REFERENCES sales(localId) ON DELETE CASCADE
            )
            """.trimIndent(),
        )
    }
}
