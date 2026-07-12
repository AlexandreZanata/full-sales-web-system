package com.fullsales.seller.app.ui.commerces.registrations

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.allowsInternetOnlyActions
import com.fullsales.seller.shared.model.CommerceRegistration
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class MyRegistrationsUiState(
    val items: List<CommerceRegistration> = emptyList(),
    val refreshing: Boolean = false,
    val error: String? = null,
    val connectivity: ConnectivityState = ConnectivityState.Offline,
) {
    val isOffline: Boolean get() = !connectivity.allowsInternetOnlyActions()
    val isEmpty: Boolean get() = !refreshing && error == null && items.isEmpty()
}

class MyRegistrationsViewModel(
    private val apiClient: SellerApiClient,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(MyRegistrationsUiState())
    val state: StateFlow<MyRegistrationsUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            networkMonitor.connectivity.collect { connectivity ->
                _state.update { it.copy(connectivity = connectivity) }
            }
        }
        refresh()
    }

    fun refresh() {
        viewModelScope.launch {
            if (!networkMonitor.isOnline()) {
                _state.update { it.copy(refreshing = false) }
                return@launch
            }
            _state.update { it.copy(refreshing = true, error = null) }
            runCatching { apiClient.listRegistrations(limit = 50) }
                .onSuccess { page -> _state.update { it.copy(items = page.data, refreshing = false) } }
                .onFailure { err ->
                    _state.update {
                        it.copy(refreshing = false, error = err.message ?: "Load failed")
                    }
                }
        }
    }
}
