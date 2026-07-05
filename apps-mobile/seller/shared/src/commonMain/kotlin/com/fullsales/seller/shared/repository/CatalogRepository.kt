package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import kotlinx.coroutines.flow.Flow

interface CatalogRepository {
    fun observeCommerces(): Flow<List<Commerce>>
    fun observeProducts(): Flow<List<Product>>
    suspend fun replaceCommerces(commerces: List<Commerce>)
    suspend fun replaceProducts(products: List<Product>)
}
