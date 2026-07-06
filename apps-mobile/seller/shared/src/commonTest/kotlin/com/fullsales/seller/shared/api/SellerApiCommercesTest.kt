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
    fun listCommerces_sendsAuthAndCursorQuery() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("Bearer seller-token", request.authHeader())
            assertEquals("/v1/commerces", request.url.encodedPath)
            assertTrue(request.url.encodedQuery.contains("limit=20"))
            assertTrue(
                request.url.encodedQuery.contains("filter[active]=true") ||
                    request.url.encodedQuery.contains("filter%5Bactive%5D=true"),
            )
            HttpStatusCode.OK to """{"data":[],"pagination":{"next_cursor":null,"has_more":false,"limit":20}}"""
        }
        val client = testClient(engine = recorder.engine())
        client.listCommerces(limit = 20, active = true)
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
    fun listCommerceAddresses_parsesAddressTypeField() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/commerces/c1/addresses", request.url.encodedPath)
            assertTrue(request.url.encodedQuery.contains("limit=100"))
            HttpStatusCode.OK to """
                {"data":[{"id":"a1","addressType":"Delivery","street":"Rua A","number":"1","city":"SP","state":"SP","postalCode":"01000","isPrimary":true}],"pagination":{"next_cursor":null,"has_more":false,"limit":100}}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val addresses = client.listCommerceAddresses("c1")
        assertEquals("Delivery", addresses.single().type)
        assertTrue(addresses.single().isPrimary)
    }

    @Test
    fun listCommerceAddresses_requiresAuthHeader() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("Bearer seller-token", request.authHeader())
            assertEquals("/v1/commerces/c1/addresses", request.url.encodedPath)
            HttpStatusCode.OK to """{"data":[],"pagination":{"next_cursor":null,"has_more":false,"limit":100}}"""
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
