package com.fullsales.seller.shared.api

import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import io.ktor.client.engine.mock.respond
import io.ktor.client.request.HttpRequestData
import io.ktor.http.HttpHeaders
import io.ktor.http.HttpStatusCode
import io.ktor.http.headersOf
import kotlinx.serialization.json.Json

internal class RecordedMockEngine(
    private val handler: (HttpRequestData) -> Pair<HttpStatusCode, String>,
) {
    val requests = mutableListOf<HttpRequestData>()

    fun engine(): MockEngine = MockEngine { request ->
        requests.add(request)
        val (status, body) = handler(request)
        respond(
            content = body,
            status = status,
            headers = headersOf(HttpHeaders.ContentType, "application/json"),
        )
    }
}

internal fun testJson(): Json = defaultSellerJson()

internal fun testClient(
    token: String? = "seller-token",
    tokenProvider: AuthTokenProvider = AuthTokenProvider { token },
    refreshHandler: TokenRefreshHandler? = null,
    engine: MockEngine,
): SellerApiClient {
    val http = HttpClient(engine) {
        installSellerDefaults(tokenProvider, refreshHandler, testJson())
    }
    return SellerApiClient(http, baseUrl = "http://test/v1", json = testJson())
}

internal fun HttpRequestData.authHeader(): String? =
    headers[HttpHeaders.Authorization]

internal fun HttpRequestData.pathAndQuery(): String =
    "${url.encodedPath}${url.encodedQuery?.let { "?$it" } ?: ""}"
