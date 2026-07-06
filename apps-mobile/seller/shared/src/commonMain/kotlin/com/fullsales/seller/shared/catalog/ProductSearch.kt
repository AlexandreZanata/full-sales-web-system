package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Product

fun productMatchesSearch(product: Product, query: String): Boolean {
    val term = query.trim().lowercase()
    if (term.isEmpty()) return true
    return product.name.lowercase().contains(term) ||
        product.sku.lowercase().contains(term)
}

fun filterProductsBySearch(products: List<Product>, query: String): List<Product> =
    products
        .asSequence()
        .filter { productMatchesSearch(it, query) }
        .sortedBy { it.name.lowercase() }
        .toList()
