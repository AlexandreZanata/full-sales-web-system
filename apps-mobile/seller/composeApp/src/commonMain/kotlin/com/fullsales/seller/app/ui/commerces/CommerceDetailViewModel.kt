package com.fullsales.seller.app.ui.commerces

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.catalog.CommerceDetailLoader
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CommerceAddressUi
import com.fullsales.seller.shared.model.toUiModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class CommerceDetailUiState(
    val loading: Boolean = true,
    val errorCode: String? = null,
    val commerce: Commerce? = null,
    val addresses: List<CommerceAddressUi> = emptyList(),
)

class CommerceDetailViewModel(
    private val loader: CommerceDetailLoader,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(CommerceDetailUiState())
    val state: StateFlow<CommerceDetailUiState> = _state.asStateFlow()

    fun load(commerceId: String) {
        viewModelScope.launch {
            _state.value = CommerceDetailUiState(loading = true)
            runCatching {
                val loaded = loader.load(commerceId, networkMonitor.isOnline())
                CommerceDetailUiState(
                    commerce = loaded.commerce,
                    addresses = loaded.addresses.map { it.toUiModel() },
                    loading = false,
                )
            }.onSuccess { _state.value = it }
                .onFailure { error ->
                    _state.value = CommerceDetailUiState(
                        loading = false,
                        errorCode = mapError(error),
                    )
                }
        }
    }

    private fun mapError(error: Throwable): String = when {
        error is ApiException -> error.detail.code
        error.message == "COMMERCE_NOT_FOUND" -> "COMMERCE_NOT_FOUND"
        else -> "LOAD_FAILED"
    }
}
