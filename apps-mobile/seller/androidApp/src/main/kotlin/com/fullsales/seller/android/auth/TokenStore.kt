package com.fullsales.seller.android.auth

import android.content.Context
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.fullsales.seller.app.platform.SellerTokenStore
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

class TokenStore(context: Context) : SellerTokenStore {
    private val appContext = context.applicationContext
    private val prefs = openPrefs(appContext)

    override fun getAccessToken(): String? = prefs.getString(KEY_ACCESS, null)

    override fun getRefreshToken(): String? = prefs.getString(KEY_REFRESH, null)

    override fun saveTokens(accessToken: String, refreshToken: String) {
        prefs.edit()
            .putString(KEY_ACCESS, accessToken)
            .putString(KEY_REFRESH, refreshToken)
            .apply()
    }

    override fun clear() {
        prefs.edit().clear().apply()
    }

    private companion object {
        const val PREFS_NAME = "seller_secure_tokens"
        const val KEY_ACCESS = "access_token"
        const val KEY_REFRESH = "refresh_token"

        fun openPrefs(context: Context) = try {
            createEncrypted(context)
        } catch (_: Exception) {
            // Keystore/prefs desync after reinstall — wipe and recreate.
            context.deleteSharedPreferences(PREFS_NAME)
            createEncrypted(context)
        }

        fun createEncrypted(context: Context) = EncryptedSharedPreferences.create(
            context,
            PREFS_NAME,
            MasterKey.Builder(context).setKeyScheme(MasterKey.KeyScheme.AES256_GCM).build(),
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM,
        )
    }
}
