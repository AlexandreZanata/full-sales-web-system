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
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.resolveListEmptyReason
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class MyRegistrationsUiState(
    val items: List<CommerceRegistration> = emptyList(),
    val refreshing: Boolean = false,
    val connectivity: ConnectivityState = ConnectivityState.Offline,
    val snackbarCode: String? = null,
    val everSynced: Boolean = false,
    val refreshFailed: Boolean = false,
) {
    val isOffline: Boolean get() = !connectivity.allowsInternetOnlyActions()

    val emptyReason: ListEmptyReason? get() = resolveListEmptyReason(
        hasLocalRows = items.isNotEmpty(),
        everSynced = everSynced,
        isOnline = !isOffline,
        refreshFailed = refreshFailed,
    )
}

/**
 * LocalStore-first registrations list (Phase 16C/16F).
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
            _state.update {
                it.copy(everSynced = registrationRepository.getLastRegistrationsSyncEpochMs() != null)
            }
        }
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
            _state.update { it.copy(refreshing = true, refreshFailed = false) }
            val ok = syncCoordinator.pullRegistrations()
            val synced = registrationRepository.getLastRegistrationsSyncEpochMs() != null
            _state.update {
                val keepCacheFail = !ok && it.items.isNotEmpty()
                it.copy(
                    refreshing = false,
                    everSynced = synced || it.everSynced,
                    refreshFailed = keepCacheFail,
                    snackbarCode = if (keepCacheFail) "REFRESH_FAILED" else null,
                )
            }
        }
    }
}
