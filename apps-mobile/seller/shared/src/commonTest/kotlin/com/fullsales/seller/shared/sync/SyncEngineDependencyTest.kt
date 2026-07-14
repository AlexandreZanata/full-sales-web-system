package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.sales.SaleActionResult
import com.fullsales.seller.shared.sales.SaleActionSubmitter
import com.fullsales.seller.shared.sales.buildSaleDetailFromLocal
import com.fullsales.seller.shared.api.SellerApiClient
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import io.ktor.client.engine.mock.respond
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertNotNull
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

/**
 * T-16-09 — create outbox + confirm dependsOn drains in order when Online.
 */
class SyncEngineDependencyTest {
    private val catalog = FakeCatalogRepository()
    private val sales = FakeSaleRepository()
    private val outbox = FakeOutboxRepository()
    private val transport = RecordingTransport()
    private val writer = OfflineSaleWriter(sales, outbox)

    @Test
    fun given_createThenConfirmDependsOn_when_process_then_confirmRunsAfterCreate() = runTest {
        catalog.seed(
            Product("p1", "Widget", "W-1", 10.0, "BRL", true),
            Commerce("c1", "Acme Ltd", "Acme", true),
        )
        val sale = writer.createSale(
            CreateSaleRequest("c1", listOf(CreateSaleItem("p1", 1)), "cash"),
            10.0,
        )
        val detail = buildSaleDetailFromLocal(sales.getSale(sale.localId)!!, emptyList(), emptyList())
        val submitter = SaleActionSubmitter(unusedApi(), sales, outbox)
        assertIs<SaleActionResult.Success>(submitter.confirm(detail, online = false))

        val confirm = outbox.all.single { it.id.endsWith(":confirm") }
        assertEquals("${sale.localId}:create", confirm.dependsOnOutboxId)
        assertEquals(SyncEntityType.Sale, confirm.entityType)
        assertEquals(LocalSaleStatus.Confirmed, sales.getSale(sale.localId)?.status)

        transport.stub(
            "${sale.localId}:create",
            SyncHttpResult(SyncHttpOutcome.Success, remoteId = "srv-chain-1"),
        )
        transport.stub(
            "${sale.localId}:confirm",
            SyncHttpResult(SyncHttpOutcome.Success, remoteId = "srv-chain-1"),
        )

        val result = SyncEngine(outbox, sales, transport, FakeTokenRefresher()).processOutbox()

        assertEquals(2, result.processedCount)
        assertEquals(2, transport.calls.size)
        assertEquals("/sales", transport.calls[0].path)
        assertEquals("/sales/srv-chain-1/confirm", transport.calls[1].path)
        assertEquals("srv-chain-1", sales.getSale(sale.localId)?.remoteId)
        assertEquals(LocalSaleStatus.Confirmed, sales.getSale(sale.localId)?.status)
        assertTrue(outbox.all.none { !it.completed })
    }

    @Test
    fun given_confirmDependsOnPendingCreate_when_createNotDone_then_confirmSkipped() = runTest {
        catalog.seed(
            Product("p1", "Widget", "W-1", 10.0, "BRL", true),
            Commerce("c1", "Acme Ltd", "Acme", true),
        )
        val sale = writer.createSale(
            CreateSaleRequest("c1", listOf(CreateSaleItem("p1", 1)), "cash"),
            10.0,
        )
        val detail = buildSaleDetailFromLocal(sales.getSale(sale.localId)!!, emptyList(), emptyList())
        SaleActionSubmitter(unusedApi(), sales, outbox).confirm(detail, online = false)

        transport.stub(
            "${sale.localId}:create",
            SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = "VALIDATION"),
        )
        transport.stub(
            "${sale.localId}:confirm",
            SyncHttpResult(SyncHttpOutcome.Success, remoteId = "should-not-run"),
        )

        val result = SyncEngine(outbox, sales, transport, FakeTokenRefresher()).processOutbox()

        assertEquals(1, result.processedCount)
        assertEquals(1, transport.calls.size)
        assertEquals("/sales", transport.calls.single().path)
        assertNotNull(outbox.getEntry("${sale.localId}:confirm")?.takeUnless { it.completed })
    }

    private fun unusedApi(): SellerApiClient = SellerApiClient(
        HttpClient(MockEngine { respond("", HttpStatusCode.OK) }),
        baseUrl = "http://test/v1",
    )
}
