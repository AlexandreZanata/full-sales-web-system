package com.fullsales.seller.shared.db

import android.content.Context
import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import com.fullsales.seller.shared.db.dao.CacheDao
import com.fullsales.seller.shared.db.dao.CatalogDao
import com.fullsales.seller.shared.db.dao.RegistrationDao
import com.fullsales.seller.shared.db.dao.SaleDao
import com.fullsales.seller.shared.db.dao.SyncOutboxDao
import com.fullsales.seller.shared.db.entity.CommerceAddressCacheEntity
import com.fullsales.seller.shared.db.entity.CommerceEntity
import com.fullsales.seller.shared.db.entity.ProductEntity
import com.fullsales.seller.shared.db.entity.RegistrationEntity
import com.fullsales.seller.shared.db.entity.SaleEntity
import com.fullsales.seller.shared.db.entity.SaleLineEntity
import com.fullsales.seller.shared.db.entity.StockSnapshotEntity
import com.fullsales.seller.shared.db.entity.SyncMetadataEntity
import com.fullsales.seller.shared.db.entity.SyncOutboxEntity

@Database(
    entities = [
        CommerceEntity::class,
        ProductEntity::class,
        SaleEntity::class,
        SaleLineEntity::class,
        SyncOutboxEntity::class,
        SyncMetadataEntity::class,
        StockSnapshotEntity::class,
        CommerceAddressCacheEntity::class,
        RegistrationEntity::class,
    ],
    version = 7,
    exportSchema = false,
)
abstract class SellerDatabase : RoomDatabase() {
    abstract fun catalogDao(): CatalogDao
    abstract fun saleDao(): SaleDao
    abstract fun syncOutboxDao(): SyncOutboxDao
    abstract fun cacheDao(): CacheDao
    abstract fun registrationDao(): RegistrationDao

    companion object {
        fun build(context: Context, name: String = "seller.db"): SellerDatabase =
            Room.databaseBuilder(context, SellerDatabase::class.java, name)
                .addMigrations(
                    SellerMigrations.MIGRATION_4_5,
                    SellerMigrations.MIGRATION_5_6,
                    SellerMigrations.MIGRATION_6_7,
                )
                .fallbackToDestructiveMigrationFrom(1, 2, 3)
                .build()

        fun inMemory(context: Context): SellerDatabase =
            Room.inMemoryDatabaseBuilder(context, SellerDatabase::class.java).build()
    }
}
