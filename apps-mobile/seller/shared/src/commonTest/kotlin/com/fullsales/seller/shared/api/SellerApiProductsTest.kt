package com.fullsales.seller.shared.api

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest
import io.ktor.http.HttpStatusCode

class SellerApiProductsTest {
    @Test
    fun listProducts_defaultsActiveFilterAndCursorEnvelope() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("Bearer seller-token", request.authHeader())
            assertEquals("/v1/products", request.url.encodedPath)
            assertTrue(request.url.encodedQuery.contains("filter"))
            assertTrue(request.url.encodedQuery.contains("active"))
            assertTrue(request.url.encodedQuery.contains("limit=50"))
            HttpStatusCode.OK to """
                {"data":[],"pagination":{"next_cursor":null,"has_more":false,"limit":50}}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        client.listProducts()
    }

    @Test
    fun getProduct_hitsDetailPath() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/products/p1", request.url.encodedPath)
            HttpStatusCode.OK to """
                {"id":"p1","name":"Widget","sku":"W-1","priceAmount":1000,"priceCurrency":"BRL","active":true,"categoryName":"Tools","unitOfMeasure":"Unit"}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val product = client.getProduct("p1")
        assertEquals("Widget", product.name)
        assertEquals("Tools", product.categoryName)
        assertEquals("Unit", product.unitOfMeasure)
    }

    @Test
    fun getStockBalance_hitsInventoryPath() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/inventory/products/p1/balance", request.url.encodedPath)
            HttpStatusCode.OK to """{"productId":"p1","available":5,"asOf":"2026-07-05T12:00:00Z"}"""
        }
        val client = testClient(engine = recorder.engine())
        val balance = client.getStockBalance("p1")
        assertEquals(5, balance.available)
    }
}
