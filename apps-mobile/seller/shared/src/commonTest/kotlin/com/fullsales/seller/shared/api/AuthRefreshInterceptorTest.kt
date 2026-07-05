package com.fullsales.seller.shared.api

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlinx.coroutines.test.runTest
import io.ktor.http.HttpStatusCode

class AuthRefreshInterceptorTest {
    @Test
    fun retriesProtectedRequestOnceAfter401WhenRefreshSucceeds() = runTest {
        var settingsCalls = 0
        var refreshCalls = 0
        var accessToken = "expired-token"
        val recorder = RecordedMockEngine { request ->
            when (request.url.encodedPath) {
                "/v1/auth/refresh" -> {
                    refreshCalls++
                    HttpStatusCode.OK to
                        """{"accessToken":"new-token","refreshToken":"new-refresh","expiresIn":3600}"""
                }
                "/v1/settings" -> {
                    settingsCalls++
                    if (settingsCalls == 1) {
                        HttpStatusCode.Unauthorized to """{"error":{"code":"UNAUTHORIZED","message":"Expired","correlationId":"00000000-0000-0000-0000-000000000000"}}"""
                    } else {
                        HttpStatusCode.OK to """{"displayName":"Acme Sales"}"""
                    }
                }
                else -> HttpStatusCode.NotFound to "{}"
            }
        }
        val refreshHandler = TokenRefreshHandler {
            accessToken = "new-token"
            true
        }
        val client = testClient(
            tokenProvider = AuthTokenProvider { accessToken },
            refreshHandler = refreshHandler,
            engine = recorder.engine(),
        )
        assertEquals("Acme Sales", client.getSettings().displayName)
        assertEquals(2, settingsCalls)
        assertEquals(0, refreshCalls)
        assertEquals("Bearer new-token", recorder.requests.last().authHeader())
    }

    @Test
    fun doesNotRetryAuthRefreshEndpointOn401() = runTest {
        var refreshCalls = 0
        val recorder = RecordedMockEngine { request ->
            refreshCalls++
            HttpStatusCode.Unauthorized to
                """{"error":{"code":"INVALID_CREDENTIALS","message":"Bad refresh","correlationId":"00000000-0000-0000-0000-000000000000"}}"""
        }
        val client = testClient(
            token = null,
            refreshHandler = TokenRefreshHandler { error("must not refresh") },
            engine = recorder.engine(),
        )
        runCatching { client.refresh("stale-refresh") }
        assertEquals(1, refreshCalls)
    }

    @Test
    fun doesNotRetryWhenRefreshFails() = runTest {
        var settingsCalls = 0
        val recorder = RecordedMockEngine { request ->
            when (request.url.encodedPath) {
                "/v1/settings" -> {
                    settingsCalls++
                    HttpStatusCode.Unauthorized to """{"error":{"code":"UNAUTHORIZED","message":"Expired","correlationId":"00000000-0000-0000-0000-000000000000"}}"""
                }
                else -> HttpStatusCode.NotFound to "{}"
            }
        }
        val client = testClient(
            refreshHandler = TokenRefreshHandler { false },
            engine = recorder.engine(),
        )
        runCatching { client.getSettings() }
        assertEquals(1, settingsCalls)
        assertNull(recorder.requests.singleOrNull { it.url.encodedPath == "/v1/auth/refresh" })
    }
}
