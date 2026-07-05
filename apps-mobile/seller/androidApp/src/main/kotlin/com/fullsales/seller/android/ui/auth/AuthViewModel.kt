package com.fullsales.seller.android.ui.auth

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.android.auth.TokenStore
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.auth.SellerRoleGateResult
import com.fullsales.seller.shared.auth.gateSellerAccessToken
import com.fullsales.seller.shared.i18n.AuthErrorCode
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class AuthUiState(
    val isAuthenticated: Boolean = false,
    val loading: Boolean = false,
    val error: AuthErrorCode? = null,
    val errorDetail: String? = null,
)

class AuthViewModel(
    private val apiClient: SellerApiClient,
    private val tokenStore: TokenStore,
) : ViewModel() {
    private val _state = MutableStateFlow(AuthUiState(isAuthenticated = hasValidSession()))
    val state: StateFlow<AuthUiState> = _state.asStateFlow()

    init {
        restoreSession()
    }

    fun restoreSession() {
        _state.value = _state.value.copy(isAuthenticated = hasValidSession(), error = null, errorDetail = null)
    }

    fun login(email: String, password: String, onSuccess: () -> Unit) {
        viewModelScope.launch {
            _state.value = _state.value.copy(loading = true, error = null, errorDetail = null)
            runCatching {
                val response = apiClient.login(email.trim(), password)
                when (val gate = gateSellerAccessToken(response.accessToken)) {
                    is SellerRoleGateResult.Accepted -> {
                        tokenStore.saveTokens(response.accessToken, response.refreshToken)
                        _state.value = AuthUiState(isAuthenticated = true)
                        onSuccess()
                    }
                    SellerRoleGateResult.NotSeller -> {
                        tokenStore.clear()
                        _state.value = AuthUiState(error = AuthErrorCode.SELLER_REQUIRED)
                    }
                    SellerRoleGateResult.InvalidToken -> {
                        tokenStore.clear()
                        _state.value = AuthUiState(error = AuthErrorCode.INVALID_SESSION)
                    }
                }
            }.onFailure { error ->
                _state.value = AuthUiState(error = mapLoginError(error))
            }
            _state.value = _state.value.copy(loading = false)
        }
    }

    fun logout(onLoggedOut: () -> Unit) {
        viewModelScope.launch {
            runCatching { apiClient.logout() }
            tokenStore.clear()
            _state.value = AuthUiState(isAuthenticated = false)
            onLoggedOut()
        }
    }

    private fun hasValidSession(): Boolean {
        val token = tokenStore.getAccessToken() ?: return false
        return gateSellerAccessToken(token) is SellerRoleGateResult.Accepted
    }

    private fun mapLoginError(error: Throwable): AuthErrorCode = when (error) {
        is ApiException -> when (error.detail.code) {
            "INVALID_CREDENTIALS" -> AuthErrorCode.INVALID_CREDENTIALS
            "FORBIDDEN" -> AuthErrorCode.SELLER_REQUIRED
            "RATE_LIMITED" -> AuthErrorCode.RATE_LIMITED
            else -> AuthErrorCode.GENERIC
        }
        else -> AuthErrorCode.LOGIN_FAILED
    }
}
