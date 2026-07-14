package com.fullsales.seller.app.ui.settings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.SiteSettingsRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class SettingsUiState(
    val displayName: String? = null,
    val logoUrl: String? = null,
    val loading: Boolean = false,
)

class SettingsViewModel(
    private val apiClient: SellerApiClient,
    private val settingsRepository: SiteSettingsRepository,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(SettingsUiState())
    val state: StateFlow<SettingsUiState> = _state.asStateFlow()
    private var fetchedAtEpochMs: Long = 0L

    fun loadIfStale(force: Boolean = false) {
        val now = currentEpochMs()
        if (!force && now - fetchedAtEpochMs < STALE_MS && _state.value.displayName != null) return
        viewModelScope.launch {
            _state.value = _state.value.copy(loading = true)
            settingsRepository.get()?.let { snap ->
                apply(snap.settings, snap.syncedAtEpochMs)
            }
            if (!networkMonitor.isOnline()) {
                _state.updateLoadingFalse()
                return@launch
            }
            runCatching { apiClient.getSettings() }
                .onSuccess { settings ->
                    settingsRepository.upsert(settings, now)
                    apply(settings, now)
                }
                .onFailure { _state.updateLoadingFalse() }
        }
    }

    private fun apply(settings: SiteSettings, now: Long) {
        fetchedAtEpochMs = now
        _state.value = SettingsUiState(
            displayName = settings.displayName,
            logoUrl = settings.logoUrl,
            loading = false,
        )
    }

    private fun MutableStateFlow<SettingsUiState>.updateLoadingFalse() {
        value = value.copy(loading = false)
    }

    private companion object {
        const val STALE_MS = 5 * 60 * 1000L
    }
}
