package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.SyncOutboxEntry
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
        val sale = writer.createSale(saleRequest(), 20.0)

        val stored = sales.getSale(sale.localId)
        assertNotNull(stored)
        assertEquals(LocalSaleStatus.PendingSync, stored.status)
        assertEquals(1, outbox.all.count { !it.completed })
        assertEquals(sale.idempotencyKey, outbox.all.first().idempotencyKey)
    }

    @Test
    fun s2_networkSync_postsSaleAndStoresRemoteId() = runTest {
        seedCatalog()
        val sale = writer.createSale(saleRequest(), 20.0)
        transport.nextResult = SyncHttpResult(SyncHttpOutcome.Success, remoteId = "srv-99")

        val result = engine().processOutbox()

        assertEquals(1, result.processedCount)
        assertEquals(1, transport.calls.size)
        assertEquals("POST", transport.calls.first().method)
        assertEquals("/sales", transport.calls.first().path)
        assertEquals("srv-99", sales.getSale(sale.localId)?.remoteId)
        assertEquals(LocalSaleStatus.Synced, sales.getSale(sale.localId)?.status)
    }

    @Test
    fun s3_confirmInsufficientStock_marksSyncFailed() = runTest {
        seedCatalog()
        val local = sales.createLocalSale(saleRequest(), 10.0)
        outbox.enqueue(
            SyncOutboxEntry(
                id = "${local.localId}:confirm",
                aggregateId = local.localId,
                method = "POST",
                path = "/sales/remote-2/confirm",
                bodyJson = "{}",
                idempotencyKey = "${local.localId}:confirm",
                createdAtEpochMs = 1L,
            ),
        )
        transport.stub(
            "${local.localId}:confirm",
            SyncHttpResult(SyncHttpOutcome.InsufficientStock, errorCode = "INSUFFICIENT_STOCK"),
        )

        engine().processOutbox()

        assertEquals(LocalSaleStatus.SyncFailed, sales.getSale(local.localId)?.status)
        assertEquals("INSUFFICIENT_STOCK", sales.getSale(local.localId)?.syncFailureReason)
    }

    @Test
    fun s4_retrySameIdempotencyKey_serverDedupes() = runTest {
        seedCatalog()
        val sale = writer.createSale(saleRequest(), 20.0)
        val idempotencyKey = outbox.all.first().idempotencyKey
        transport.nextResult = SyncHttpResult(SyncHttpOutcome.Success, remoteId = "srv-dedup")
        engine().processOutbox()
        outbox.enqueue(
            SyncOutboxEntry(
                id = "${sale.localId}:retry",
                aggregateId = sale.localId,
                method = "POST",
                path = "/sales",
                bodyJson = "{}",
                idempotencyKey = idempotencyKey,
                createdAtEpochMs = 2L,
            ),
        )
        engine().processOutbox()

        assertTrue(transport.calls.all { it.idempotencyKey == idempotencyKey })
        assertEquals("srv-dedup", sales.getSale(sale.localId)?.remoteId)
    }

    @Test
    fun s5_networkError_stopsEarlyAndIncrementsAttempts() = runTest {
        seedCatalog()
        writer.createSale(saleRequest(), 20.0)
        transport.nextResult = SyncHttpResult(SyncHttpOutcome.NetworkError, errorCode = "timeout")

        val result = engine().processOutbox()

        assertTrue(result.stoppedEarly)
        assertEquals(1, outbox.all.first { !it.completed }.attempts)
    }

    @Test
    fun given_attemptsAtMax_when_processOutbox_then_marksSyncFailed() = runTest {
        seedCatalog()
        val sale = writer.createSale(saleRequest(), 20.0)
        val entry = outbox.all.first()
        repeat(5) { outbox.markFailed(entry.id, "timeout") }

        engine().processOutbox()

        assertEquals(LocalSaleStatus.SyncFailed, sales.getSale(sale.localId)?.status)
        assertTrue(outbox.all.first().completed)
    }

    private fun seedCatalog() {
        catalog.seed(
            Product("p1", "Widget", "W-1", 10.0, "BRL", true),
            Commerce("c1", "Acme Ltd", "Acme", true),
        )
    }

    private fun saleRequest() = CreateSaleRequest(
        commerceId = "c1",
        paymentMethod = "cash",
        items = listOf(CreateSaleItem("p1", 2)),
    )
}
