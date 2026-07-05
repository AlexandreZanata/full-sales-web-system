package com.fullsales.seller.shared.api

import io.ktor.client.HttpClient
import io.ktor.client.HttpClientConfig
import io.ktor.client.plugins.HttpTimeout
import io.ktor.client.plugins.api.createClientPlugin
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.http.HttpHeaders
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json

class SellerAuthConfig {
    lateinit var tokenProvider: AuthTokenProvider
}

private val SellerAuthPlugin = createClientPlugin("SellerAuth", ::SellerAuthConfig) {
    val provider = pluginConfig.tokenProvider
    onRequest { request, _ ->
        val path = request.url.buildString()
        if (!path.contains("/auth/login") && !path.contains("/auth/refresh")) {
            val token = provider.accessToken()
            if (token != null) {
                request.headers[HttpHeaders.Authorization] = "Bearer $token"
            } else {
                request.headers.remove(HttpHeaders.Authorization)
            }
        }
    }
}

fun defaultSellerJson(): Json = Json {
    ignoreUnknownKeys = true
    isLenient = true
}

fun HttpClientConfig<*>.installSellerDefaults(
    tokenProvider: AuthTokenProvider,
    tokenRefreshHandler: TokenRefreshHandler? = null,
    json: Json = defaultSellerJson(),
) {
    install(ContentNegotiation) { json(json) }
    install(HttpTimeout) {
        requestTimeoutMillis = 30_000
        connectTimeoutMillis = 10_000
        socketTimeoutMillis = 30_000
    }
    install(SellerAuthPlugin) {
        this.tokenProvider = tokenProvider
    }
    if (tokenRefreshHandler != null) {
        install(SellerRefreshPlugin) {
            this.refreshHandler = tokenRefreshHandler
            this.tokenProvider = tokenProvider
        }
    }
}

fun createSellerHttpClient(
    tokenProvider: AuthTokenProvider,
    tokenRefreshHandler: TokenRefreshHandler? = null,
    json: Json = defaultSellerJson(),
    config: HttpClientConfig<*>.() -> Unit = {},
): HttpClient = HttpClient {
    installSellerDefaults(tokenProvider, tokenRefreshHandler, json)
    config()
}
