package com.fullsales.seller.shared.db

import android.content.Context
import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import com.fullsales.seller.shared.db.dao.CatalogDao
import com.fullsales.seller.shared.db.dao.SaleDao
import com.fullsales.seller.shared.db.dao.SyncOutboxDao
import com.fullsales.seller.shared.db.entity.CommerceEntity
import com.fullsales.seller.shared.db.entity.ProductEntity
import com.fullsales.seller.shared.db.entity.SaleEntity
import com.fullsales.seller.shared.db.entity.SaleLineEntity
import com.fullsales.seller.shared.db.entity.SyncOutboxEntity
import com.fullsales.seller.shared.db.entity.SyncMetadataEntity

@Database(
    entities = [
        CommerceEntity::class,
        ProductEntity::class,
        SaleEntity::class,
        SaleLineEntity::class,
        SyncOutboxEntity::class,
        SyncMetadataEntity::class,
    ],
    version = 2,
    exportSchema = false,
)
abstract class SellerDatabase : RoomDatabase() {
    abstract fun catalogDao(): CatalogDao
    abstract fun saleDao(): SaleDao
    abstract fun syncOutboxDao(): SyncOutboxDao

    companion object {
        fun build(context: Context, name: String = "seller.db"): SellerDatabase =
            Room.databaseBuilder(context, SellerDatabase::class.java, name)
                .fallbackToDestructiveMigration()
                .build()

        fun inMemory(context: Context): SellerDatabase =
            Room.inMemoryDatabaseBuilder(context, SellerDatabase::class.java).build()
    }
}
