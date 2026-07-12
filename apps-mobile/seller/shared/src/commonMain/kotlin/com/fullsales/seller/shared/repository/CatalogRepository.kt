package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import kotlinx.coroutines.flow.Flow

interface CatalogRepository {
    fun observeCommerces(): Flow<List<Commerce>>
    fun observeProducts(): Flow<List<Product>>
    suspend fun getCommerce(id: String): Commerce?
    suspend fun getProduct(id: String): Product?
    suspend fun replaceCommerces(commerces: List<Commerce>)
    suspend fun replaceProducts(products: List<Product>)
    suspend fun getLastCatalogSyncEpochMs(): Long?
    suspend fun setLastCatalogSyncEpochMs(epochMs: Long)
}
