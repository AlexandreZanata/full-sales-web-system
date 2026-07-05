package com.fullsales.seller.android.auth

import android.content.Context
import com.fullsales.seller.shared.sync.SyncTokenRefresher

/** ponytail: Phase 53 wires refresh via TokenStore + SellerApiClient.refresh */
class NoOpTokenRefresher : SyncTokenRefresher {
    override suspend fun refreshToken(): Boolean = false
}

class TokenStore(context: Context) {
    private val prefs = context.getSharedPreferences(PREFS, Context.MODE_PRIVATE)

    fun getAccessToken(): String? = prefs.getString(KEY_ACCESS, null)

    fun getRefreshToken(): String? = prefs.getString(KEY_REFRESH, null)

    fun saveTokens(accessToken: String, refreshToken: String) {
        prefs.edit()
            .putString(KEY_ACCESS, accessToken)
            .putString(KEY_REFRESH, refreshToken)
            .apply()
    }

    fun clear() {
        prefs.edit().clear().apply()
    }

    private companion object {
        const val PREFS = "seller_auth"
        const val KEY_ACCESS = "access_token"
        const val KEY_REFRESH = "refresh_token"
    }
}
