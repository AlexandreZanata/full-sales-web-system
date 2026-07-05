package com.fullsales.seller.shared.api

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlinx.coroutines.test.runTest
import io.ktor.http.HttpStatusCode

class SellerApiAuthTest {
    @Test
    fun login_doesNotAttachBearerToken() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertNull(request.authHeader())
            assertEquals("/v1/auth/login", request.url.encodedPath)
            HttpStatusCode.OK to """{"accessToken":"a","refreshToken":"r","expiresIn":3600}"""
        }
        val client = testClient(engine = recorder.engine())
        client.login("seller@test.com", "secret123")
    }

    @Test
    fun parseApiError_readsNestedErrorObject() {
        val body = """
            {"error":{"code":"INVALID_CREDENTIALS","message":"Bad login","correlationId":"abc-123"}}
        """.trimIndent()
        val detail = parseApiError(body, testJson())
        requireNotNull(detail)
        assertEquals("INVALID_CREDENTIALS", detail.code)
        assertEquals("Bad login", detail.message)
        assertEquals("abc-123", detail.correlationId)
    }

    @Test
    fun getSettings_attachesBearerToken() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("Bearer seller-token", request.authHeader())
            assertEquals("/v1/settings", request.url.encodedPath)
            HttpStatusCode.OK to """{"displayName":"Acme Sales"}"""
        }
        val client = testClient(engine = recorder.engine())
        assertEquals("Acme Sales", client.getSettings().displayName)
    }
}
