package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.i18n.SyncChipStatus
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleDisplayStatus
import kotlin.test.Test
import kotlin.test.assertEquals

class SaleDetailModelTest {
    @Test
    fun buildSaleDetailFromRemote_usesCommerceLegalName() {
        val commerce = Commerce(
            id = "c1",
            legalName = "Acme Comercio LTDA",
            tradeName = "Acme Store",
            active = true,
        )
        val sale = Sale(
            id = "s1",
            commerceId = "c1",
            driverId = "d1",
            status = "Pending",
            paymentMethod = "Cash",
            totalAmount = 1000.0,
            totalCurrency = "BRL",
        )
        val detail = buildSaleDetailFromRemote(sale, local = null, commerces = listOf(commerce), products = emptyList())
        assertEquals("Acme Comercio LTDA", detail.commerceName)
    }

    @Test
    fun buildSaleDetailFromLocal_showsPendingChipWhenOutboxPendingAfterConfirm() {
        val local = LocalSale(
            localId = "l1",
            idempotencyKey = "idem",
            commerceId = "c1",
            paymentMethod = "cash",
            status = LocalSaleStatus.Confirmed,
            totalAmount = 100.0,
            items = emptyList(),
            createdAtEpochMs = 1L,
            remoteId = "r1",
        )
        val detail = buildSaleDetailFromLocal(local, emptyList(), emptyList(), hasPendingOutbox = true)
        assertEquals(SyncChipStatus.PendingSync, detail.syncChip)
        assertEquals(SaleDisplayStatus.Confirmed, detail.status)
    }
}
