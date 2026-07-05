package com.fullsales.seller.app.ui.commerces

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CommerceAddressUi
import com.fullsales.seller.shared.model.toUiModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class CommerceDetailUiState(
    val loading: Boolean = true,
    val error: String? = null,
    val commerce: Commerce? = null,
    val addresses: List<CommerceAddressUi> = emptyList(),
)

class CommerceDetailViewModel(
    private val apiClient: SellerApiClient,
) : ViewModel() {
    private val _state = MutableStateFlow(CommerceDetailUiState())
    val state: StateFlow<CommerceDetailUiState> = _state.asStateFlow()

    fun load(commerceId: String) {
        viewModelScope.launch {
            _state.value = CommerceDetailUiState(loading = true)
            runCatching {
                val commerce = apiClient.getCommerce(commerceId)
                val addresses = apiClient.listCommerceAddresses(commerceId).map { it.toUiModel() }
                CommerceDetailUiState(commerce = commerce, addresses = addresses, loading = false)
            }.onSuccess { _state.value = it }
                .onFailure { error ->
                    _state.value = CommerceDetailUiState(
                        loading = false,
                        error = mapError(error),
                    )
                }
        }
    }

    private fun mapError(error: Throwable): String = when (error) {
        is ApiException -> when (error.detail.code) {
            "COMMERCE_NOT_FOUND" -> "Commerce not found"
            "UNAUTHORIZED" -> "Session expired"
            else -> error.detail.message
        }
        else -> error.message ?: "Failed to load commerce"
    }
}
