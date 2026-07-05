package com.fullsales.seller.shared.api

import io.ktor.client.plugins.api.createClientPlugin
import io.ktor.client.plugins.api.Send
import io.ktor.http.HttpHeaders
import io.ktor.http.HttpStatusCode
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class SellerRefreshConfig {
    lateinit var refreshHandler: TokenRefreshHandler
    lateinit var tokenProvider: AuthTokenProvider
}

private val refreshMutex = Mutex()

private fun isAuthPath(path: String): Boolean =
    path.contains("/auth/login") || path.contains("/auth/refresh")

internal val SellerRefreshPlugin = createClientPlugin("SellerRefresh", ::SellerRefreshConfig) {
    val handler = pluginConfig.refreshHandler
    val tokenProvider = pluginConfig.tokenProvider
    on(Send) { request ->
        val call = proceed(request)
        val path = call.request.url.encodedPath
        if (call.response.status != HttpStatusCode.Unauthorized || isAuthPath(path)) {
            return@on call
        }
        val refreshed = refreshMutex.withLock { handler.refreshTokens() }
        if (!refreshed) return@on call
        tokenProvider.accessToken()?.let { token ->
            request.headers[HttpHeaders.Authorization] = "Bearer $token"
        }
        proceed(request)
    }
}
