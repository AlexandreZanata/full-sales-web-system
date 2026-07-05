package com.fullsales.seller.shared.auth

actual class SecureTokenStore actual constructor() {
    private val keychain = KeychainTokenStore()

    actual fun getAccessToken(): String? = keychain.getAccessToken()

    actual fun getRefreshToken(): String? = keychain.getRefreshToken()

    actual fun saveTokens(accessToken: String, refreshToken: String) {
        keychain.saveTokens(accessToken, refreshToken)
    }

    actual fun clear() {
        keychain.clear()
    }
}
