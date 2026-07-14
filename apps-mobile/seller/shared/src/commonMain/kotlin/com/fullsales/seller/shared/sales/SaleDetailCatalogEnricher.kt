package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.catalog.toProduct
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product

class SaleDetailCatalogEnricher(
    private val apiClient: SellerApiClient,
) {
    suspend fun enrich(
        commerceId: String,
        productIds: List<String>,
        commerces: List<Commerce>,
        products: List<Product>,
        online: Boolean,
    ): Pair<List<Commerce>, List<Product>> {
        if (!online) return commerces to products
        val commerceById = commerces.associateBy { it.id }.toMutableMap()
        val productById = products.associateBy { it.id }.toMutableMap()
        if (commerceId !in commerceById) {
            runCatching { apiClient.getCommerce(commerceId) }
                .getOrNull()
                ?.let { commerceById[it.id] = it }
        }
        productIds.distinct()
            .filter { it.isNotBlank() && it !in productById }
            .forEach { id ->
                runCatching { apiClient.getProduct(id) }
                    .getOrNull()
                    ?.let { productById[it.id] = it.toProduct() }
            }
        return commerceById.values.toList() to productById.values.toList()
    }
}
