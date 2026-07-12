package com.fullsales.seller.shared.db.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.fullsales.seller.shared.db.entity.CommerceAddressCacheEntity
import com.fullsales.seller.shared.db.entity.StockSnapshotEntity

@Dao
interface CacheDao {
    @Query("SELECT * FROM stock_snapshots WHERE productId = :productId LIMIT 1")
    suspend fun getStock(productId: String): StockSnapshotEntity?

    @Query("SELECT * FROM stock_snapshots")
    suspend fun listStock(): List<StockSnapshotEntity>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertStock(item: StockSnapshotEntity)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertStockAll(items: List<StockSnapshotEntity>)

    @Query("SELECT * FROM commerce_address_cache WHERE commerceId = :commerceId LIMIT 1")
    suspend fun getAddresses(commerceId: String): CommerceAddressCacheEntity?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertAddresses(item: CommerceAddressCacheEntity)
}
