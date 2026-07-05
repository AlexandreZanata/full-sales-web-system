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
            provider.accessToken()?.let { token ->
                request.headers.append(HttpHeaders.Authorization, "Bearer $token")
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
}

fun createSellerHttpClient(
    tokenProvider: AuthTokenProvider,
    json: Json = defaultSellerJson(),
    config: HttpClientConfig<*>.() -> Unit = {},
): HttpClient = HttpClient {
    installSellerDefaults(tokenProvider, json)
    config()
}
