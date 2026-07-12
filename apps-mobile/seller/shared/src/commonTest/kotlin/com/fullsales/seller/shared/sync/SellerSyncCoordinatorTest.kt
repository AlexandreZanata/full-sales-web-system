package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlinx.coroutines.test.runTest

/**
 * Contract (Phase 14B): catalog pull failure must not abort outbox push.
 */
class SellerSyncCoordinatorTest {
    @Test
    fun given_catalogPullThrows_when_syncPullAndPush_then_outboxStillProcessed() = runTest {
        val catalog = FakeCatalogRepository()
        val sales = FakeSaleRepository()
        val outbox = FakeOutboxRepository()
        val transport = RecordingTransport()
        val pullClient = FakeCatalogPullClient().apply { throwOnFetch = true }
        val writer = OfflineSaleWriter(sales, outbox)
        val sale = writer.createSale(
            CreateSaleRequest(
                commerceId = "c1",
                paymentMethod = "cash",
                items = listOf(CreateSaleItem("p1", 1)),
            ),
            10.0,
        )
        transport.nextResult = SyncHttpResult(SyncHttpOutcome.Success, remoteId = "srv-1")
        val coordinator = SellerSyncCoordinator(
            CatalogPullSync(catalog, pullClient),
            SyncEngine(outbox, sales, transport, FakeTokenRefresher()),
        )

        val result = coordinator.syncPullAndPush()

        assertEquals(1, result.processedCount)
        assertEquals(1, transport.calls.size)
        val stored = sales.getSale(sale.localId)
        assertNotNull(stored)
        assertEquals("srv-1", stored.remoteId)
        assertEquals(LocalSaleStatus.Synced, stored.status)
    }
}
