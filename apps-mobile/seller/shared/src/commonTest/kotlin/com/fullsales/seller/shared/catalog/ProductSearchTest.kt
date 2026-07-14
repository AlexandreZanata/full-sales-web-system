package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Product
import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ProductSearchTest {
    private val widget = Product(
        id = "p1",
        name = "Economy Pack",
        sku = "SEED-003",
        priceAmount = 10.0,
        priceCurrency = "BRL",
        active = true,
    )

    @Test
    fun matches_name_case_insensitive() {
        assertTrue(productMatchesSearch(widget, "economy"))
        assertTrue(productMatchesSearch(widget, "PACK"))
    }

    @Test
    fun matches_sku_case_insensitive() {
        assertTrue(productMatchesSearch(widget, "seed-003"))
    }

    @Test
    fun empty_query_matches_all() {
        assertTrue(productMatchesSearch(widget, ""))
        assertTrue(productMatchesSearch(widget, "   "))
    }

    @Test
    fun no_match_returns_false() {
        assertFalse(productMatchesSearch(widget, "unknown"))
    }

    @Test
    fun filter_sorts_by_name() {
        val alpha = widget.copy(id = "a", name = "Alpha")
        val beta = widget.copy(id = "b", name = "Beta")
        val result = filterProductsBySearch(listOf(beta, alpha), "")
        assertTrue(result.first().name == "Alpha")
    }

    @Test
    fun sale_picker_blank_query_returns_empty_not_full_catalog() {
        val catalog = listOf(widget, widget.copy(id = "p2", name = "Other", sku = "SKU-2"))
        assertTrue(filterProductsForSalePickerSearch(catalog, "").isEmpty())
        assertTrue(filterProductsForSalePickerSearch(catalog, "   ").isEmpty())
    }

    @Test
    fun sale_picker_search_caps_results() {
        val catalog = (1..30).map { i ->
            widget.copy(id = "p$i", name = "Item $i", sku = "SKU-$i")
        }
        val result = filterProductsForSalePickerSearch(catalog, "Item", limit = 5)
        assertTrue(result.size == 5)
    }
}
