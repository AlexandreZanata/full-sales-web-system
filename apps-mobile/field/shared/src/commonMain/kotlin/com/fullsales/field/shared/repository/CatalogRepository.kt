package com.fullsales.field.shared.repository

import com.fullsales.field.shared.model.Commerce
import com.fullsales.field.shared.model.Product
import com.fullsales.field.shared.model.StockBalance

interface CatalogRepository {
    suspend fun listActiveCommerces(): List<Commerce>
    suspend fun listActiveProducts(): List<Product>
    suspend fun getStockBalance(productId: String): StockBalance?
    suspend fun replaceCommerces(commerces: List<Commerce>)
    suspend fun replaceProducts(products: List<Product>)
    suspend fun upsertStockBalance(balance: StockBalance)
    suspend fun listProductIds(): List<String>
    suspend fun getLastCatalogSyncEpochMs(): Long?
    suspend fun setLastCatalogSyncEpochMs(epochMs: Long)
}
