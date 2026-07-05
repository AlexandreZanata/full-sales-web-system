package com.fullsales.seller.android.auth

import android.content.Context
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.TokenRefreshHandler
import com.fullsales.seller.shared.auth.SellerRoleGateResult
import com.fullsales.seller.shared.auth.gateSellerAccessToken
import com.fullsales.seller.shared.sync.SyncTokenRefresher

class SellerTokenRefresher(
    private val tokenStore: TokenStore,
    private val authApiClient: SellerApiClient,
) : SyncTokenRefresher, TokenRefreshHandler {
    override suspend fun refreshToken(): Boolean = refreshTokens()

    override suspend fun refreshTokens(): Boolean {
        val refresh = tokenStore.getRefreshToken() ?: return false
        return runCatching {
            val response = authApiClient.refresh(refresh)
            when (gateSellerAccessToken(response.accessToken)) {
                is SellerRoleGateResult.Accepted -> {
                    tokenStore.saveTokens(response.accessToken, response.refreshToken)
                    true
                }
                else -> {
                    tokenStore.clear()
                    false
                }
            }
        }.getOrDefault(false)
    }
}

class TokenStore(context: Context) {
    private val prefs = EncryptedSharedPreferences.create(
        context,
        PREFS_NAME,
        MasterKey.Builder(context).setKeyScheme(MasterKey.KeyScheme.AES256_GCM).build(),
        EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
        EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM,
    )

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
        const val PREFS_NAME = "seller_secure_tokens"
        const val KEY_ACCESS = "access_token"
        const val KEY_REFRESH = "refresh_token"
    }
}
