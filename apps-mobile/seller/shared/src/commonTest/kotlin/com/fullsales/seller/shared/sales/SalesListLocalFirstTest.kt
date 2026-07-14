package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleDisplayStatus
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import com.fullsales.seller.shared.sync.FakeCatalogPullClient
import com.fullsales.seller.shared.sync.FakeSaleRepository
import com.fullsales.seller.shared.sync.PullSalesSync
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest

/**
 * T-16-06 — LocalStore remains source of truth when pull/API fails.
 */
class SalesListLocalFirstTest {
    @Test
    fun given_localStoreSales_when_pullFails_then_listStillReturnsLocal() = runTest {
        val sales = FakeSaleRepository()
        sales.upsertFromRemoteSales(
            listOf(
                Sale(
                    id = "cached-1",
                    commerceId = "c1",
                    driverId = "d1",
                    status = "Pending",
                    paymentMethod = "pix",
                    totalAmount = 50.0,
                    totalCurrency = "BRL",
                    items = listOf(SaleItem("p1", 1, 50.0, "BRL", 50.0)),
                    createdAt = "2000",
                ),
            ),
        )
        sales.setLastSalesSyncEpochMs(1L)
        val client = FakeCatalogPullClient().apply { throwOnSalesFetch = true }
        runCatching { PullSalesSync(sales, client).pullSales() }

        val items = localSalesToListItems(sales.observeSales().first())
        assertEquals(1, items.size)
        assertEquals("cached-1", items.single().navigationId)
        assertEquals(SaleDisplayStatus.Pending, items.single().status)
        assertTrue(sales.getLastSalesSyncEpochMs() == 1L)
    }

    @Test
    fun given_pendingLocalAndMirroredRemote_when_mapLocal_then_bothAppear() {
        val local = listOf(
            LocalSale(
                localId = "local-pending",
                remoteId = null,
                idempotencyKey = "k1",
                commerceId = "c1",
                paymentMethod = "cash",
                status = LocalSaleStatus.PendingSync,
                totalAmount = 10.0,
                createdAtEpochMs = 100L,
                origin = SaleOrigin.Local,
            ),
            LocalSale(
                localId = "remote-1",
                remoteId = "remote-1",
                idempotencyKey = "remote-1",
                commerceId = "c1",
                paymentMethod = "cash",
                status = LocalSaleStatus.Confirmed,
                totalAmount = 20.0,
                createdAtEpochMs = 200L,
                origin = SaleOrigin.RemoteMirror,
            ),
        )
        val items = localSalesToListItems(local)
        assertEquals(2, items.size)
        assertEquals(SaleDisplayStatus.PendingSync, items.first().status)
        assertEquals("local-pending", items.first().navigationId)
    }
}
