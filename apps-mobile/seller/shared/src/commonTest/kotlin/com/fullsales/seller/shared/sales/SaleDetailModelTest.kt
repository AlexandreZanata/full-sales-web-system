package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.i18n.SellerLocale
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.SaleDisplayStatus
import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class SaleDetailModelTest {
    @Test
    fun showActions_hiddenWhenConfirmed() {
        assertFalse(showSaleDetailActions(SaleDisplayStatus.Confirmed, "remote-1"))
    }

    @Test
    fun showActions_visibleWhenPendingWithRemoteId() {
        assertTrue(showSaleDetailActions(SaleDisplayStatus.Pending, "remote-1"))
    }

    @Test
    fun showActions_hiddenWhenPendingWithoutRemoteId() {
        assertFalse(showSaleDetailActions(SaleDisplayStatus.Pending, null))
    }

    @Test
    fun saleActionError_insufficientStock_en() {
        val messages = SellerStrings.forLocale(SellerLocale.En)
        assertTrue(SellerStrings.saleActionError(messages, "INSUFFICIENT_STOCK").contains("Insufficient stock"))
    }
}
