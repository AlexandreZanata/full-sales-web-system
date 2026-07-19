package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Product
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Contract: product search uses smart matching (accents, tokens, small typos).
 */
class ProductSearchTest {
    private val coca = Product(
        id = "p1",
        name = "Coca-Cola 2L",
        sku = "SEED-001",
        priceAmount = 8.99,
        priceCurrency = "BRL",
        active = true,
    )

    private val widget = Product(
        id = "p2",
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
    fun given_typo_coco_when_search_then_matches_coca_cola() {
        assertTrue(productMatchesSearch(coca, "coco"))
    }

    @Test
    fun given_second_word_cola_when_search_then_matches_coca_cola() {
        assertTrue(productMatchesSearch(coca, "cola"))
    }

    @Test
    fun filter_sorts_better_matches_first() {
        val other = widget.copy(id = "a", name = "Cola Diet")
        val result = filterProductsBySearch(listOf(other, coca), "coca cola")
        assertEquals("Coca-Cola 2L", result.first().name)
    }

    @Test
    fun sale_picker_blank_query_returns_empty_not_full_catalog() {
        val catalog = listOf(widget, widget.copy(id = "p3", name = "Other", sku = "SKU-2"))
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
