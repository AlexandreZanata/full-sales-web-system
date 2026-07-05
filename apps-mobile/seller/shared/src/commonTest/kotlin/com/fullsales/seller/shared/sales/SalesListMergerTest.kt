package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleDisplayStatus
import kotlin.test.Test
import kotlin.test.assertEquals

class SalesListMergerTest {
    @Test
    fun merge_prefersRemoteStatusWhenLocalSaleSynced() {
        val remote = listOf(
            sale(id = "remote-1", status = "Confirmed", createdAtMs = 2000L),
        )
        val local = listOf(
            localSale(
                localId = "local-1",
                remoteId = "remote-1",
                status = LocalSaleStatus.Synced,
                createdAtMs = 1000L,
            ),
        )
        val merged = mergeSalesList(remote, local) { it?.toLong() ?: 0L }
        assertEquals(1, merged.size)
        assertEquals(SaleDisplayStatus.Confirmed, merged.single().status)
        assertEquals("remote-1", merged.single().navigationId)
    }

    @Test
    fun merge_pendingSyncSaleAppearsAtTop() {
        val remote = listOf(
            sale(id = "remote-old", status = "Pending", createdAtMs = 9000L),
        )
        val local = listOf(
            localSale(
                localId = "local-pending",
                remoteId = null,
                status = LocalSaleStatus.PendingSync,
                createdAtMs = 1000L,
            ),
        )
        val merged = mergeSalesList(remote, local) { it?.toLong() ?: 0L }
        assertEquals(2, merged.size)
        assertEquals(SaleDisplayStatus.PendingSync, merged.first().status)
        assertEquals("local-pending", merged.first().navigationId)
    }

    @Test
    fun merge_dedupesByRemoteId() {
        val remote = listOf(sale(id = "remote-1", status = "Pending", createdAtMs = 5000L))
        val local = listOf(
            localSale("local-1", "remote-1", LocalSaleStatus.Synced, 5000L),
        )
        assertEquals(1, mergeSalesList(remote, local) { it?.toLong() ?: 0L }.size)
    }

    private fun sale(id: String, status: String, createdAtMs: Long) = Sale(
        id = id,
        commerceId = "c1",
        driverId = "d1",
        status = status,
        paymentMethod = "cash",
        totalAmount = 100.0,
        totalCurrency = "BRL",
        createdAt = createdAtMs.toString(),
    )

    private fun localSale(
        localId: String,
        remoteId: String?,
        status: LocalSaleStatus,
        createdAtMs: Long,
    ) = LocalSale(
        localId = localId,
        remoteId = remoteId,
        idempotencyKey = "key-$localId",
        commerceId = "c1",
        paymentMethod = "cash",
        status = status,
        totalAmount = 50.0,
        createdAtEpochMs = createdAtMs,
    )
}
