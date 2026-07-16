package com.fullsales.seller.shared.api

import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import io.ktor.client.engine.mock.respond
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

class SellerApiProbeTest {
    @Test
    fun probeReachable_true_on_health_ok() = runTest {
        val client = SellerApiClient(
            HttpClient(MockEngine { respond("""{"status":"ok"}""", HttpStatusCode.OK) }),
            baseUrl = "http://example.test/v1",
        )
        assertTrue(client.probeReachable())
    }

    @Test
    fun probeReachable_false_on_connection_error() = runTest {
        val client = SellerApiClient(
            HttpClient(MockEngine { error("connection refused") }),
            baseUrl = "http://example.test/v1",
        )
        assertFalse(client.probeReachable())
    }
}
