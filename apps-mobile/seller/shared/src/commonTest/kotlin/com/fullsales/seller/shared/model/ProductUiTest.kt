package com.fullsales.seller.shared.model

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ProductUiTest {
    @Test
    fun formatMoneyMinorUnits_formatsBrlFromCentavos() {
        assertEquals("R$ 2.500,00", formatMoneyMinorUnits(250_000))
        assertEquals("R$ 1.200,00", formatMoneyMinorUnits(120_000))
        assertEquals("R$ 450,00", formatMoneyMinorUnits(45_000))
    }

    @Test
    fun formatProductPrice_convertsDoubleMinorUnits() {
        assertEquals("R$ 10,50", formatProductPrice(1050.0, "BRL"))
    }

    @Test
    fun stockBadgeLabel_showsUnavailableWhenZero() {
        assertEquals("Unavailable", stockBadgeLabel(0))
        assertTrue(isStockUnavailable(0))
    }

    @Test
    fun stockBadgeLabel_showsAvailableCount() {
        assertEquals("Available: 86", stockBadgeLabel(86))
        assertFalse(isStockUnavailable(86))
    }
}
