package com.fullsales.seller.app.platform.ios

import com.fullsales.seller.app.platform.SellerTokenStore
import com.fullsales.seller.shared.auth.SecureTokenStore

class KeychainSellerTokenStore : SellerTokenStore {
    private val store = SecureTokenStore()

    override fun getAccessToken(): String? = store.getAccessToken()

    override fun getRefreshToken(): String? = store.getRefreshToken()

    override fun saveTokens(accessToken: String, refreshToken: String) {
        store.saveTokens(accessToken, refreshToken)
    }

    override fun clear() {
        store.clear()
    }
}
