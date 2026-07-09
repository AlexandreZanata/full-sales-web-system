package com.fullsales.seller.shared.sales

import kotlin.test.Test
import kotlin.test.assertEquals

class VisualStockTest {
    @Test
    fun visualStockRemaining_whenLinesReserveQty_thenSubtractsFromAvailable() {
        val lines = listOf(
            CreateSaleLineInput(productId = "p1", quantityText = "2"),
            CreateSaleLineInput(productId = "p2", quantityText = "1"),
            CreateSaleLineInput(productId = "p1", quantityText = "3"),
        )
        assertEquals(74, visualStockRemaining(79, lines, "p1"))
        assertEquals(78, visualStockRemaining(79, lines, "p2"))
    }

    @Test
    fun visualStockRemaining_whenReservedExceedsAvailable_thenFloorsAtZero() {
        val lines = listOf(CreateSaleLineInput(productId = "p1", quantityText = "100"))
        assertEquals(0, visualStockRemaining(79, lines, "p1"))
    }

    @Test
    fun visualStockRemaining_whenStockUnknown_thenReturnsNull() {
        val lines = listOf(CreateSaleLineInput(productId = "p1", quantityText = "2"))
        assertEquals(null, visualStockRemaining(null, lines, "p1"))
    }
}
