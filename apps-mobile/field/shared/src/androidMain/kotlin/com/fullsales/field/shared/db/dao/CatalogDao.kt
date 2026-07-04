package com.fullsales.field.shared.db.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.fullsales.field.shared.db.entity.CommerceEntity
import com.fullsales.field.shared.db.entity.ProductEntity
import com.fullsales.field.shared.db.entity.StockBalanceEntity
import com.fullsales.field.shared.db.entity.SyncMetadataEntity

@Dao
interface CatalogDao {
    @Query("SELECT * FROM commerces WHERE active = 1 ORDER BY legalName")
    suspend fun listActiveCommerces(): List<CommerceEntity>

    @Query("SELECT * FROM products WHERE active = 1 ORDER BY name")
    suspend fun listActiveProducts(): List<ProductEntity>

    @Query("SELECT * FROM stock_balances WHERE productId = :productId LIMIT 1")
    suspend fun getStockBalance(productId: String): StockBalanceEntity?

    @Query("SELECT id FROM products")
    suspend fun listProductIds(): List<String>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertCommerces(items: List<CommerceEntity>)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertProducts(items: List<ProductEntity>)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertStockBalance(item: StockBalanceEntity)

    @Query("DELETE FROM commerces")
    suspend fun clearCommerces()

    @Query("DELETE FROM products")
    suspend fun clearProducts()

    @Query("SELECT value FROM sync_metadata WHERE `key` = :key LIMIT 1")
    suspend fun getMetadata(key: String): String?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertMetadata(item: SyncMetadataEntity)
}
