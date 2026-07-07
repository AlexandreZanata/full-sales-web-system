package com.fullsales.seller.app.ui.settings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.model.currentEpochMs
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
) : ViewModel() {
    private val _state = MutableStateFlow(SettingsUiState())
    val state: StateFlow<SettingsUiState> = _state.asStateFlow()
    private var fetchedAtEpochMs: Long = 0L

    fun loadIfStale(force: Boolean = false) {
        val now = currentEpochMs()
        if (!force && now - fetchedAtEpochMs < STALE_MS && _state.value.displayName != null) return
        viewModelScope.launch {
            _state.value = _state.value.copy(loading = true)
            runCatching { apiClient.getSettings() }
                .onSuccess { settings -> apply(settings, now) }
                .onFailure {
                    _state.value = _state.value.copy(loading = false)
                }
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

    private companion object {
        const val STALE_MS = 5 * 60 * 1000L
    }
}
