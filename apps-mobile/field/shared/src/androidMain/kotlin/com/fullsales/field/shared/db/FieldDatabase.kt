package com.fullsales.field.shared.db

import android.content.Context
import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import com.fullsales.field.shared.db.dao.CatalogDao
import com.fullsales.field.shared.db.dao.SaleDao
import com.fullsales.field.shared.db.dao.SyncOutboxDao
import com.fullsales.field.shared.db.entity.CommerceEntity
import com.fullsales.field.shared.db.entity.ProductEntity
import com.fullsales.field.shared.db.entity.SaleEntity
import com.fullsales.field.shared.db.entity.SaleLineEntity
import com.fullsales.field.shared.db.entity.StockBalanceEntity
import com.fullsales.field.shared.db.entity.SyncMetadataEntity
import com.fullsales.field.shared.db.entity.SyncOutboxEntity

@Database(
    entities = [
        CommerceEntity::class,
        ProductEntity::class,
        StockBalanceEntity::class,
        SaleEntity::class,
        SaleLineEntity::class,
        SyncOutboxEntity::class,
        SyncMetadataEntity::class,
    ],
    version = 1,
    exportSchema = false,
)
abstract class FieldDatabase : RoomDatabase() {
    abstract fun catalogDao(): CatalogDao
    abstract fun saleDao(): SaleDao
    abstract fun syncOutboxDao(): SyncOutboxDao

    companion object {
        fun build(context: Context): FieldDatabase =
            Room.databaseBuilder(context, FieldDatabase::class.java, "field.db").build()
    }
}
