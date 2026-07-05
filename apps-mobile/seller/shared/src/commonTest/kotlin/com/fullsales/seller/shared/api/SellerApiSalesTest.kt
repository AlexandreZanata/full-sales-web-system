package com.fullsales.seller.shared.api

import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest
import io.ktor.http.HttpStatusCode

class SellerApiSalesTest {
    @Test
    fun createSale_sendsIdempotencyKeyAndAuth() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("Bearer seller-token", request.authHeader())
            assertEquals("/v1/sales", request.url.encodedPath)
            assertEquals("550e8400-e29b-41d4-a716-446655440000", request.headers["Idempotency-Key"])
            HttpStatusCode.Created to """
                {"id":"s1","commerceId":"c1","driverId":"u1","status":"Pending","paymentMethod":"Cash","totalAmount":2000,"totalCurrency":"BRL","items":[]}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val request = CreateSaleRequest(
            commerceId = "c1",
            items = listOf(CreateSaleItem("p1", 2)),
            paymentMethod = "cash",
        )
        val sale = client.createSale(request, "550e8400-e29b-41d4-a716-446655440000")
        assertEquals("Pending", sale.status)
    }

    @Test
    fun listSales_hitsPaginatedPath() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/sales", request.url.encodedPath)
            assertTrue(request.url.encodedQuery.contains("page=2"))
            HttpStatusCode.OK to """{"page":2,"pageSize":50,"total":0,"items":[]}"""
        }
        val client = testClient(engine = recorder.engine())
        client.listSales(page = 2)
    }

    @Test
    fun confirmSale_hitsConfirmPath() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/sales/s1/confirm", request.url.encodedPath)
            HttpStatusCode.OK to """
                {"id":"s1","commerceId":"c1","driverId":"u1","status":"Confirmed","paymentMethod":"Cash","totalAmount":2000,"totalCurrency":"BRL","items":[]}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        assertEquals("Confirmed", client.confirmSale("s1").status)
    }

    @Test
    fun cancelSale_hitsCancelPath() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/sales/s1/cancel", request.url.encodedPath)
            HttpStatusCode.OK to """
                {"id":"s1","commerceId":"c1","driverId":"u1","status":"Cancelled","paymentMethod":"Cash","totalAmount":2000,"totalCurrency":"BRL","items":[]}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        assertEquals("Cancelled", client.cancelSale("s1").status)
    }
}
