package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.Product
import kotlin.test.Test
import kotlin.test.assertEquals

class CreateSaleTotalTest {
    @Test
    fun calculateTotal_sumsPriceTimesQuantityInMinorUnits() {
        val products = listOf(
            product("p1", 1500.0),
            product("p2", 500.0),
        )
        val lines = listOf(
            CreateSaleLineInput("p1", "2"),
            CreateSaleLineInput("p2", "1"),
        )
        assertEquals(3500L, calculateCreateSaleTotalMinor(products, lines))
    }

    @Test
    fun calculateTotal_ignoresInvalidLines() {
        val products = listOf(product("p1", 1000.0))
        val lines = listOf(
            CreateSaleLineInput("", "2"),
            CreateSaleLineInput("p1", "0"),
            CreateSaleLineInput("p1", "abc"),
            CreateSaleLineInput("p1", "3"),
        )
        assertEquals(3000L, calculateCreateSaleTotalMinor(products, lines))
    }

    private fun product(id: String, priceMinor: Double) = Product(
        id = id,
        name = "Item",
        sku = "SKU",
        priceAmount = priceMinor,
        priceCurrency = "BRL",
        active = true,
    )
}
