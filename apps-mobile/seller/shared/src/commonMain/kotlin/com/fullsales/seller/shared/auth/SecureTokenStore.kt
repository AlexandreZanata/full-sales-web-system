package com.fullsales.seller.shared.auth

expect class SecureTokenStore() {
    fun getAccessToken(): String?
    fun getRefreshToken(): String?
    fun saveTokens(accessToken: String, refreshToken: String)
    fun clear()
}
