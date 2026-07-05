package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.sync.FakeOutboxRepository
import com.fullsales.seller.shared.sync.FakeSaleRepository
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import io.ktor.client.engine.mock.respond
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
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
    }

    @Test
    fun mapInvalidTransitionMessage() {
        assertEquals(
            "This sale can no longer be changed",
            mapSaleActionError("INVALID_SALE_TRANSITION"),
        )
    }

    private fun unusedApi(): SellerApiClient = SellerApiClient(
        HttpClient(MockEngine { respond("", HttpStatusCode.OK) }),
        baseUrl = "http://test/v1",
    )
}
