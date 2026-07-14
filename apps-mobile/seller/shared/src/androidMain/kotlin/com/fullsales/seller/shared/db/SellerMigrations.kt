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

    val MIGRATION_5_6: Migration = object : Migration(5, 6) {
        override fun migrate(db: SupportSQLiteDatabase) {
            db.execSQL(
                """
                CREATE TABLE IF NOT EXISTS registrations (
                  localId TEXT NOT NULL PRIMARY KEY,
                  remoteId TEXT,
                  cnpj TEXT NOT NULL,
                  legalName TEXT NOT NULL,
                  tradeName TEXT NOT NULL,
                  active INTEGER NOT NULL,
                  registrationStatus TEXT NOT NULL,
                  rejectionReason TEXT,
                  registrationMode TEXT,
                  contactPhone TEXT,
                  contactEmail TEXT,
                  deliveryAddressJson TEXT NOT NULL,
                  syncStatus TEXT NOT NULL,
                  syncFailureReason TEXT,
                  createdAtEpochMs INTEGER NOT NULL,
                  updatedAtEpochMs INTEGER NOT NULL,
                  idempotencyKey TEXT NOT NULL
                )
                """.trimIndent(),
            )
            db.execSQL(
                "ALTER TABLE sync_outbox ADD COLUMN entityType TEXT NOT NULL DEFAULT 'Sale'",
            )
        }
    }

    val MIGRATION_6_7: Migration = object : Migration(6, 7) {
        override fun migrate(db: SupportSQLiteDatabase) {
            db.execSQL(
                """
                CREATE TABLE IF NOT EXISTS sync_outbox_new (
                  id TEXT NOT NULL PRIMARY KEY,
                  aggregateId TEXT NOT NULL,
                  method TEXT NOT NULL,
                  path TEXT NOT NULL,
                  bodyJson TEXT NOT NULL,
                  idempotencyKey TEXT NOT NULL,
                  createdAtEpochMs INTEGER NOT NULL,
                  attempts INTEGER NOT NULL,
                  lastError TEXT,
                  completed INTEGER NOT NULL,
                  entityType TEXT NOT NULL DEFAULT 'Sale',
                  dependsOnOutboxId TEXT
                )
                """.trimIndent(),
            )
            db.execSQL(
                """
                INSERT INTO sync_outbox_new (
                  id, aggregateId, method, path, bodyJson, idempotencyKey,
                  createdAtEpochMs, attempts, lastError, completed, entityType, dependsOnOutboxId
                )
                SELECT
                  id, saleLocalId, method, path, bodyJson, idempotencyKey,
                  createdAtEpochMs, attempts, lastError, completed, entityType, NULL
                FROM sync_outbox
                """.trimIndent(),
            )
            db.execSQL("DROP TABLE sync_outbox")
            db.execSQL("ALTER TABLE sync_outbox_new RENAME TO sync_outbox")
            db.execSQL("CREATE INDEX IF NOT EXISTS index_sync_outbox_aggregateId ON sync_outbox(aggregateId)")
            db.execSQL("CREATE INDEX IF NOT EXISTS index_sync_outbox_completed ON sync_outbox(completed)")
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

    /** Minimal v5 DDL for migration contract tests (tables touched by 5→6). */
    fun createV5CoreTables(db: SupportSQLiteDatabase) {
        createV4CoreTables(db)
        MIGRATION_4_5.migrate(db)
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS sync_outbox (
              id TEXT NOT NULL PRIMARY KEY,
              saleLocalId TEXT NOT NULL,
              method TEXT NOT NULL,
              path TEXT NOT NULL,
              bodyJson TEXT NOT NULL,
              idempotencyKey TEXT NOT NULL,
              createdAtEpochMs INTEGER NOT NULL,
              attempts INTEGER NOT NULL,
              lastError TEXT,
              completed INTEGER NOT NULL
            )
            """.trimIndent(),
        )
    }

    /** Minimal v6 DDL for migration contract tests (tables touched by 6→7). */
    fun createV6CoreTables(db: SupportSQLiteDatabase) {
        createV5CoreTables(db)
        MIGRATION_5_6.migrate(db)
    }
}
