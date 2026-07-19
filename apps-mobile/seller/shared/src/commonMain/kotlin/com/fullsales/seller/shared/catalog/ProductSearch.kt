package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.search.smartSearchRank
import com.fullsales.seller.shared.search.textMatchesSmartSearch

fun productMatchesSearch(product: Product, query: String): Boolean {
    if (query.trim().isEmpty()) return true
    return textMatchesSmartSearch(product.name, query) ||
        textMatchesSmartSearch(product.sku, query)
}

fun filterProductsBySearch(products: List<Product>, query: String): List<Product> =
    products
        .asSequence()
        .filter { productMatchesSearch(it, query) }
        .sortedWith(
            compareBy<Product> { smartSearchRank("${it.name} ${it.sku}", query) }
                .thenBy { it.name.lowercase() },
        )
        .toList()

/**
 * Sale line picker: blank query shows top-sellers only (UI); typed search returns a capped list.
 * GIVEN blank or whitespace query WHEN filtering THEN empty (do not dump full catalog).
 */
fun filterProductsForSalePickerSearch(
    products: List<Product>,
    query: String,
    limit: Int = SALE_PICKER_SEARCH_LIMIT,
): List<Product> {
    if (query.trim().isEmpty()) return emptyList()
    return filterProductsBySearch(products, query).take(limit.coerceAtLeast(0))
}

const val SALE_PICKER_SEARCH_LIMIT: Int = 20
