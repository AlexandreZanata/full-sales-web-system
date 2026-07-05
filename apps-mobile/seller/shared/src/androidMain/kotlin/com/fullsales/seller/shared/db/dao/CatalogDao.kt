package com.fullsales.seller.shared.db.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import androidx.room.Transaction
import com.fullsales.seller.shared.db.entity.CommerceEntity
import com.fullsales.seller.shared.db.entity.ProductEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface CatalogDao {
    @Query("SELECT * FROM commerces WHERE active = 1 ORDER BY legalName")
    fun observeActiveCommerces(): Flow<List<CommerceEntity>>

    @Query("SELECT * FROM products WHERE active = 1 ORDER BY name")
    fun observeActiveProducts(): Flow<List<ProductEntity>>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertCommerces(items: List<CommerceEntity>)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertProducts(items: List<ProductEntity>)

    @Query("DELETE FROM commerces")
    suspend fun clearCommerces()

    @Query("DELETE FROM products")
    suspend fun clearProducts()

    @Transaction
    suspend fun replaceCommerces(items: List<CommerceEntity>) {
        clearCommerces()
        if (items.isNotEmpty()) upsertCommerces(items)
    }

    @Transaction
    suspend fun replaceProducts(items: List<ProductEntity>) {
        clearProducts()
        if (items.isNotEmpty()) upsertProducts(items)
    }
}
