package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Contract: seller-facing sale codes are visual-only, 8-char alphanumeric,
 * sequential by creation time (oldest = 00000001). Navigation still uses real ids.
 */
class SaleDisplayCodeTest {
    @Test
    fun format_padsBase36ToEightChars() {
        assertEquals("00000001", formatSaleDisplayCode(1))
        assertEquals("0000000A", formatSaleDisplayCode(10))
        assertEquals("00000010", formatSaleDisplayCode(36))
    }

    @Test
    fun localSalesToListItems_assignsSequentialCodesByCreatedAt() {
        val local = listOf(
            localSale("newer", createdAtMs = 3000L),
            localSale("oldest", createdAtMs = 1000L),
            localSale("middle", createdAtMs = 2000L),
        )
        val items = localSalesToListItems(local).associateBy { it.navigationId }
        assertEquals("00000001", items.getValue("oldest").shortId)
        assertEquals("00000002", items.getValue("middle").shortId)
        assertEquals("00000003", items.getValue("newer").shortId)
    }

    @Test
    fun saleDisplayCodes_mapsRemoteIdToSameCode() {
        val local = listOf(
            localSale("local-1", remoteId = "remote-1", createdAtMs = 1000L),
        )
        val codes = saleDisplayCodes(local)
        assertEquals("00000001", codes["local-1"])
        assertEquals("00000001", codes["remote-1"])
    }

    private fun localSale(
        localId: String,
        remoteId: String? = null,
        createdAtMs: Long,
    ) = LocalSale(
        localId = localId,
        remoteId = remoteId,
        idempotencyKey = "key-$localId",
        commerceId = "c1",
        paymentMethod = "cash",
        status = LocalSaleStatus.Synced,
        totalAmount = 50.0,
        createdAtEpochMs = createdAtMs,
    )
}
