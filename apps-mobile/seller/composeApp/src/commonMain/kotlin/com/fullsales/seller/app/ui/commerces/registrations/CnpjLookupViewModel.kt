package com.fullsales.seller.app.ui.commerces.registrations

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.allowsInternetOnlyActions
import com.fullsales.seller.shared.model.CnpjLookupResult
import com.fullsales.seller.shared.model.isValidCnpjInput
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class CnpjLookupUiState(
    val cnpj: String = "",
    val loading: Boolean = false,
    val errorCode: String? = null,
    val result: CnpjLookupResult? = null,
    val connectivity: ConnectivityState = ConnectivityState.Offline,
) {
    val lookupEnabled: Boolean get() = !loading && connectivity.allowsInternetOnlyActions()
}

class CnpjLookupViewModel(
    private val apiClient: SellerApiClient,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(CnpjLookupUiState())
    val state: StateFlow<CnpjLookupUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            networkMonitor.connectivity.collect { connectivity ->
                _state.update { it.copy(connectivity = connectivity) }
            }
        }
    }

    fun setCnpj(value: String) {
        _state.update { it.copy(cnpj = value, errorCode = null) }
    }

    fun lookup(onSuccess: (CnpjLookupResult) -> Unit) {
        val cnpj = _state.value.cnpj
        if (!isValidCnpjInput(cnpj)) {
            _state.update { it.copy(errorCode = "INVALID_CNPJ") }
            return
        }
        if (!_state.value.lookupEnabled) return
        viewModelScope.launch {
            _state.update { it.copy(loading = true, errorCode = null) }
            runCatching { apiClient.lookupCnpj(cnpj) }
                .onSuccess { result ->
                    _state.update { it.copy(loading = false, result = result) }
                    onSuccess(result)
                }
                .onFailure { err ->
                    _state.update { it.copy(loading = false, errorCode = lookupErrorCode(err)) }
                }
        }
    }
}

internal fun lookupErrorCode(err: Throwable): String {
    var current: Throwable? = err
    while (current != null) {
        val api = current as? ApiException
        if (api != null) return api.detail.code
        current = current.cause
    }
    return "LOOKUP_FAILED"
}
