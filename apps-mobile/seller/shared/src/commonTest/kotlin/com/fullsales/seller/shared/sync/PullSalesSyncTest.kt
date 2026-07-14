package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest

/**
 * T-16-03 / T-16-04 — PullSalesSync upserts remote sales with line prices; pull twice dedupes.
 */
class PullSalesSyncTest {
    @Test
    fun given_remoteSaleFixture_when_pullSales_then_localRowsAndLinePrices() = runTest {
        val sales = FakeSaleRepository()
        val client = FakeCatalogPullClient().apply {
            this.sales = listOf(remoteSale("srv-1", status = "Pending", linePrice = 12.5))
        }
        PullSalesSync(sales, client).pullSales(nowEpochMs = 55_000L)

        val stored = sales.getSale("srv-1")
        assertNotNull(stored)
        assertEquals("srv-1", stored.remoteId)
        assertEquals(SaleOrigin.RemoteMirror, stored.origin)
        assertEquals(LocalSaleStatus.Synced, stored.status)
        assertEquals(1, stored.items.size)
        assertEquals(12.5, stored.items.single().unitPriceAmount)
        assertEquals(25.0, stored.items.single().lineTotalAmount)
        assertEquals(55_000L, sales.getLastSalesSyncEpochMs())
        assertEquals(1, sales.observeSales().first().size)
    }

    @Test
    fun given_duplicateRemoteId_when_pullTwice_then_singleRow() = runTest {
        val sales = FakeSaleRepository()
        val client = FakeCatalogPullClient().apply {
            this.sales = listOf(remoteSale("srv-dup", status = "Confirmed", linePrice = 1.0))
        }
        val sync = PullSalesSync(sales, client)
        sync.pullSales(nowEpochMs = 1L)
        client.sales = listOf(remoteSale("srv-dup", status = "Confirmed", linePrice = 9.0))
        sync.pullSales(nowEpochMs = 2L)

        assertEquals(1, sales.observeSales().first().size)
        val stored = sales.getSale("srv-dup")!!
        assertEquals(LocalSaleStatus.Confirmed, stored.status)
        assertEquals(9.0, stored.items.single().unitPriceAmount)
        assertEquals(2L, sales.getLastSalesSyncEpochMs())
    }

    private fun remoteSale(id: String, status: String, linePrice: Double) = Sale(
        id = id,
        commerceId = "c1",
        driverId = "d1",
        status = status,
        paymentMethod = "cash",
        totalAmount = linePrice * 2,
        totalCurrency = "BRL",
        items = listOf(
            SaleItem(
                productId = "p1",
                quantity = 2,
                unitPriceAmount = linePrice,
                unitPriceCurrency = "BRL",
                lineTotalAmount = linePrice * 2,
            ),
        ),
        createdAt = "1000",
    )
}
