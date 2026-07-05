package com.fullsales.seller.shared.auth

// ponytail: Phase 64 replaces with Keychain-backed store
actual class SecureTokenStore actual constructor() {
    private var accessToken: String? = null
    private var refreshToken: String? = null

    actual fun getAccessToken(): String? = accessToken

    actual fun getRefreshToken(): String? = refreshToken

    actual fun saveTokens(accessToken: String, refreshToken: String) {
        this.accessToken = accessToken
        this.refreshToken = refreshToken
    }

    actual fun clear() {
        accessToken = null
        refreshToken = null
    }
}
