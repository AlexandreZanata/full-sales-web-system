package com.fullsales.field.shared.sync

import com.fullsales.field.shared.model.Commerce
import com.fullsales.field.shared.model.CreateSaleItem
import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.model.Product
import com.fullsales.field.shared.model.StockBalance
import com.fullsales.field.shared.model.SyncOutboxEntry
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

class SyncEngineTest {
    private val catalog = FakeCatalogRepository()
    private val sales = FakeSaleRepository()
    private val outbox = FakeOutboxRepository()
    private val transport = RecordingTransport()
    private val writer = OfflineSaleWriter(sales, outbox)

    private fun engine() = SyncEngine(outbox, sales, transport, FakeTokenRefresher())

    @Test
    fun s1_offlineCreateSale_queuesOutboxWithPendingSync() = runTest {
        seedCatalog()
        writer.createSale("local-1", saleRequest(), 20.0)

        val sale = sales.getSale("local-1")
        assertNotNull(sale)
        assertEquals(LocalSaleStatus.PendingSync, sale.status)
        assertEquals(1, outbox.all.count { !it.completed })
        assertEquals("local-1", outbox.all.first().idempotencyKey)
    }

    @Test
    fun s2_networkSync_postsSaleAndStoresRemoteId() = runTest {
        s1_offlineCreateSale_queuesOutboxWithPendingSync()
        transport.nextResult = SyncHttpResult(SyncHttpOutcome.Success, remoteId = "srv-99")

        val result = engine().processOutbox()

        assertEquals(1, result.processedCount)
        assertEquals(1, transport.calls.size)
        assertEquals("POST", transport.calls.first().method)
        assertEquals("/sales", transport.calls.first().path)
        assertEquals("srv-99", sales.getSale("local-1")?.remoteId)
        assertEquals(LocalSaleStatus.PendingRemote, sales.getSale("local-1")?.status)
    }

    @Test
    fun s3_confirmInsufficientStock_marksSyncFailed() = runTest {
        seedCatalog()
        sales.createOfflineSale("local-2", saleRequest(), 10.0)
        outbox.enqueue(
            SyncOutboxEntry(
                id = "local-2:confirm",
                saleLocalId = "local-2",
                method = "POST",
                path = "/sales/remote-2/confirm",
                bodyJson = "{}",
                idempotencyKey = "local-2:confirm",
                createdAtEpochMs = 1L,
            ),
        )
        transport.stub(
            "local-2:confirm",
            SyncHttpResult(SyncHttpOutcome.InsufficientStock, errorCode = "INSUFFICIENT_STOCK"),
        )

        engine().processOutbox()

        assertEquals(LocalSaleStatus.SyncFailed, sales.getSale("local-2")?.status)
    }

    @Test
    fun s4_retrySameIdempotencyKey_serverDedupes() = runTest {
        seedCatalog()
        writer.createSale("local-1", saleRequest(), 20.0)
        transport.nextResult = SyncHttpResult(SyncHttpOutcome.Success, remoteId = "srv-dedup")
        engine().processOutbox()
        outbox.enqueue(
            SyncOutboxEntry(
                id = "local-1:retry",
                saleLocalId = "local-1",
                method = "POST",
                path = "/sales",
                bodyJson = "{}",
                idempotencyKey = "local-1",
                createdAtEpochMs = 2L,
            ),
        )
        engine().processOutbox()

        assertTrue(transport.calls.all { it.idempotencyKey == "local-1" })
        assertEquals("srv-dedup", sales.getSale("local-1")?.remoteId)
    }

    private fun seedCatalog() {
        catalog.seed(
            Product("p1", "Widget", "W-1", 10.0, "BRL", true),
            Commerce("c1", "Acme Ltd", "Acme", true),
            StockBalance("p1", 5, "2026-07-04T00:00:00Z"),
        )
    }

    private fun saleRequest() = CreateSaleRequest(
        commerceId = "c1",
        paymentMethod = "cash",
        items = listOf(CreateSaleItem("p1", 2)),
    )
}
