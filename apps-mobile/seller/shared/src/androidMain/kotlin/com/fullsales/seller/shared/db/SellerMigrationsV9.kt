package com.fullsales.seller.shared.db

import androidx.room.migration.Migration
import androidx.sqlite.db.SupportSQLiteDatabase

/** Room v8→v9: persist server-assigned sale displayCode. */
object SellerMigrationsV9 {
    val MIGRATION_8_9: Migration = object : Migration(8, 9) {
        override fun migrate(db: SupportSQLiteDatabase) {
            db.execSQL("ALTER TABLE sales ADD COLUMN displayCode TEXT")
        }
    }
}
