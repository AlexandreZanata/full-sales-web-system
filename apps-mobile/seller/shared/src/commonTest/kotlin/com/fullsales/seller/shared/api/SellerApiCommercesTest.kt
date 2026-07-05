package com.fullsales.seller.shared.api

import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertNull
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

class SellerApiCommercesTest {
    @Test
    fun listCommerces_sendsAuthAndPagination() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("Bearer seller-token", request.authHeader())
            assertEquals("/v1/commerces", request.url.encodedPath)
            assertTrue(request.url.encodedQuery.contains("page=1"))
            assertTrue(request.url.encodedQuery.contains("pageSize=20"))
            assertTrue(request.url.encodedQuery.contains("active=true"))
            HttpStatusCode.OK to """{"page":1,"pageSize":20,"total":1,"items":[]}"""
        }
        val client = testClient(engine = recorder.engine())
        client.listCommerces(page = 1, pageSize = 20, active = true)
        assertEquals(1, recorder.requests.size)
    }

    @Test
    fun getCommerce_wrongPathFails() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/commerces/wrong-id", request.url.encodedPath)
            HttpStatusCode.NotFound to """
                {"error":{"code":"COMMERCE_NOT_FOUND","message":"Not found","correlationId":"00000000-0000-0000-0000-000000000001"}}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val error = assertFailsWith<ApiException> { client.getCommerce("wrong-id") }
        assertEquals("COMMERCE_NOT_FOUND", error.detail.code)
    }

    @Test
    fun listCommerceAddresses_requiresAuthHeader() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("Bearer seller-token", request.authHeader())
            assertEquals("/v1/commerces/c1/addresses", request.url.encodedPath)
            HttpStatusCode.OK to "[]"
        }
        val client = testClient(engine = recorder.engine())
        client.listCommerceAddresses("c1")
    }

    @Test
    fun listCommerces_missingAuthHeaderStillCalls() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertNull(request.authHeader())
            HttpStatusCode.Unauthorized to """
                {"error":{"code":"UNAUTHORIZED","message":"Missing token","correlationId":"00000000-0000-0000-0000-000000000002"}}
            """.trimIndent()
        }
        val client = testClient(token = null, engine = recorder.engine())
        assertFailsWith<ApiException> { client.listCommerces() }
    }
}
