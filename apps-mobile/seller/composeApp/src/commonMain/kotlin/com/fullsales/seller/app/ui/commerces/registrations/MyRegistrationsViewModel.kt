package com.fullsales.seller.app.ui.commerces.registrations

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.allowsInternetOnlyActions
import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.toCommerceRegistration
import com.fullsales.seller.shared.repository.RegistrationRepository
import com.fullsales.seller.shared.sync.SellerSyncCoordinator
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
    val snackbarCode: String? = null,
) {
    val isOffline: Boolean get() = !connectivity.allowsInternetOnlyActions()
    val isEmpty: Boolean get() = !refreshing && error == null && items.isEmpty()
}

/**
 * LocalStore-first registrations list (Phase 16C).
 */
class MyRegistrationsViewModel(
    private val registrationRepository: RegistrationRepository,
    private val syncCoordinator: SellerSyncCoordinator,
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
        viewModelScope.launch {
            registrationRepository.observeRegistrations().collect { local ->
                _state.update {
                    it.copy(items = local.map { row -> row.toCommerceRegistration() })
                }
            }
        }
        refresh()
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun refresh() {
        viewModelScope.launch {
            if (!networkMonitor.isOnline()) {
                _state.update {
                    it.copy(
                        refreshing = false,
                        snackbarCode = if (it.items.isNotEmpty()) "OFFLINE" else null,
                    )
                }
                return@launch
            }
            _state.update { it.copy(refreshing = true, error = null) }
            val ok = syncCoordinator.pullRegistrations()
            _state.update {
                it.copy(
                    refreshing = false,
                    error = if (!ok && it.items.isEmpty()) "Load failed" else null,
                    snackbarCode = if (!ok && it.items.isNotEmpty()) "REFRESH_FAILED" else null,
                )
            }
        }
    }
}
