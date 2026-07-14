package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.i18n.SellerLocale
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.sync.FakeOutboxRepository
import com.fullsales.seller.shared.sync.FakeSaleRepository
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import io.ktor.client.engine.mock.respond
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

class SaleActionSubmitterTest {
    private val sales = FakeSaleRepository()
    private val outbox = FakeOutboxRepository()

    @Test
    fun offlineConfirm_enqueuesOutboxPost() = runTest {
        val local = sales.createLocalSale(
            com.fullsales.seller.shared.model.CreateSaleRequest(
                commerceId = "c1",
                paymentMethod = "cash",
                items = listOf(com.fullsales.seller.shared.model.CreateSaleItem("p1", 1)),
            ),
            1000.0,
        )
        sales.setRemoteId(local.localId, "remote-9", LocalSaleStatus.Synced)
        val detail = buildSaleDetailFromLocal(
            sales.getSale(local.localId)!!,
            emptyList(),
            emptyList(),
        )
        val submitter = SaleActionSubmitter(unusedApi(), sales, outbox)
        val result = submitter.confirm(detail, online = false)
        assertTrue(result is SaleActionResult.Success)
        assertEquals("/sales/remote-9/confirm", outbox.all.first().path)
        assertEquals(LocalSaleStatus.Confirmed, sales.getSale(local.localId)?.status)
    }

    @Test
    fun given_pendingCreateOutbox_when_confirmOffline_then_chainsDependsOn() = runTest {
        val writer = OfflineSaleWriter(sales, outbox)
        val local = writer.createSale(
            com.fullsales.seller.shared.model.CreateSaleRequest(
                commerceId = "c1",
                paymentMethod = "cash",
                items = listOf(com.fullsales.seller.shared.model.CreateSaleItem("p1", 1)),
            ),
            1000.0,
        )
        val detail = buildSaleDetailFromLocal(sales.getSale(local.localId)!!, emptyList(), emptyList())
        val result = SaleActionSubmitter(unusedApi(), sales, outbox).confirm(detail, online = false)
        assertIs<SaleActionResult.Success>(result)
        val confirm = outbox.all.single { it.id.endsWith(":confirm") }
        assertEquals("${local.localId}:create", confirm.dependsOnOutboxId)
        assertEquals("/sales/${local.localId}/confirm", confirm.path)
        assertEquals(LocalSaleStatus.Confirmed, sales.getSale(local.localId)?.status)
    }

    @Test
    fun given_noRemoteIdAndNoCreateOutbox_when_confirm_then_blocked() = runTest {
        val local = sales.createLocalSale(
            com.fullsales.seller.shared.model.CreateSaleRequest(
                commerceId = "c1",
                paymentMethod = "cash",
                items = listOf(com.fullsales.seller.shared.model.CreateSaleItem("p1", 1)),
            ),
            1000.0,
        )
        sales.updateStatus(local.localId, LocalSaleStatus.PendingSync)
        val detail = buildSaleDetailFromLocal(sales.getSale(local.localId)!!, emptyList(), emptyList())
        val result = SaleActionSubmitter(unusedApi(), sales, outbox).confirm(detail, online = false)
        assertTrue(result is SaleActionResult.Failure)
        assertEquals("NO_REMOTE_ID", (result as SaleActionResult.Failure).code)
    }

    @Test
    fun invalidTransitionMessage_en() {
        val messages = SellerStrings.forLocale(SellerLocale.En)
        assertEquals(
            "This sale can no longer be changed",
            SellerStrings.saleActionError(messages, "INVALID_SALE_TRANSITION"),
        )
    }

    @Test
    fun awaitingSyncMessage_ptBr() {
        val messages = SellerStrings.forLocale(SellerLocale.PtBr)
        assertEquals(
            "Aguardando sincronização",
            SellerStrings.saleActionError(messages, "NO_REMOTE_ID"),
        )
    }

    private fun unusedApi(): SellerApiClient = SellerApiClient(
        HttpClient(MockEngine { respond("", HttpStatusCode.OK) }),
        baseUrl = "http://test/v1",
    )
}
