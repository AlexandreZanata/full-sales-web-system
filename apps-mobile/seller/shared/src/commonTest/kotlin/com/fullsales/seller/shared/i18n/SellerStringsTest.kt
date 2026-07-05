package com.fullsales.seller.shared.i18n

import kotlin.test.Test
import kotlin.test.assertEquals

class SellerStringsTest {
    @Test
    fun defaultLocaleIsPtBr() {
        assertEquals(SellerLocale.PtBr, SellerLocale.DEFAULT)
        val pt = SellerStrings.forLocale(SellerLocale.DEFAULT)
        assertEquals("Confirmar venda", pt.sales.confirm)
    }

    @Test
    fun switchToEnChangesSaleConfirmLabel() {
        val en = SellerStrings.forLocale(SellerLocale.En)
        assertEquals("Confirm sale", en.sales.confirm)
    }
}
