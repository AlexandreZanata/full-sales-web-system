package com.fullsales.seller.shared.api

fun interface TokenRefreshHandler {
    suspend fun refreshTokens(): Boolean
}
