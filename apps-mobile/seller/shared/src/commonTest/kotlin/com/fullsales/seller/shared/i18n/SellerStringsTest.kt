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

    @Test
    fun paymentMethod_mapsApiPascalCaseToLocale() {
        val pt = SellerStrings.forLocale(SellerLocale.PtBr)
        assertEquals("Dinheiro", SellerStrings.paymentMethod(pt, "Cash"))
        assertEquals("PIX", SellerStrings.paymentMethod(pt, "Pix"))
    }

    @Test
    fun offlineBanner_enAndPtBr() {
        val en = SellerStrings.forLocale(SellerLocale.En)
        val pt = SellerStrings.forLocale(SellerLocale.PtBr)
        assertEquals("You're offline", en.offline.bannerTitle)
        assertEquals("Você está offline", pt.offline.bannerTitle)
        assertEquals("Servidor indisponível. Usando dados locais.", pt.offline.bannerServer)
    }
}
