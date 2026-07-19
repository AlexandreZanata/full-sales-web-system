package com.fullsales.seller.app.ui.profile

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.PatchSellerProfileRequest
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class ProfileUiState(
    val loading: Boolean = true,
    val saving: Boolean = false,
    val contactPhone: String = "",
    val operatingRegion: String = "",
    val publicCode: String = "",
    val shareLinkActive: Boolean = true,
    val errorCode: String? = null,
    val snackbarCode: String? = null,
)

class ProfileViewModel(
    private val apiClient: SellerApiClient,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(ProfileUiState())
    val state: StateFlow<ProfileUiState> = _state.asStateFlow()

    fun load() {
        viewModelScope.launch {
            _state.update { it.copy(loading = true, errorCode = null) }
            if (!networkMonitor.canAttemptNetwork()) {
                _state.update { it.copy(loading = false, errorCode = "OFFLINE") }
                return@launch
            }
            runCatching { apiClient.getMySellerProfile() }
                .onSuccess { profile ->
                    _state.update {
                        it.copy(
                            loading = false,
                            contactPhone = profile.contactPhone.orEmpty(),
                            operatingRegion = profile.operatingRegion.orEmpty(),
                            publicCode = profile.publicCode.orEmpty(),
                            shareLinkActive = profile.shareLinkActive,
                        )
                    }
                }
                .onFailure {
                    _state.update { it.copy(loading = false, errorCode = "LOAD_FAILED") }
                }
        }
    }

    fun setContactPhone(value: String) {
        _state.update { it.copy(contactPhone = value.filter { ch -> ch.isDigit() }) }
    }

    fun setOperatingRegion(value: String) {
        _state.update { it.copy(operatingRegion = value) }
    }

    fun setShareLinkActive(value: Boolean) {
        _state.update { it.copy(shareLinkActive = value) }
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun save() {
        viewModelScope.launch {
            val phoneDigits = _state.value.contactPhone.filter { it.isDigit() }
            if (phoneDigits.isNotEmpty() && (phoneDigits.length < 10 || phoneDigits.length > 15)) {
                _state.update { it.copy(snackbarCode = "PHONE_INVALID") }
                return@launch
            }
            if (!networkMonitor.canAttemptNetwork()) {
                _state.update { it.copy(snackbarCode = "OFFLINE") }
                return@launch
            }
            _state.update { it.copy(saving = true) }
            val request = PatchSellerProfileRequest(
                operatingRegion = _state.value.operatingRegion.trim().ifEmpty { null },
                contactPhone = phoneDigits,
                shareLinkActive = _state.value.shareLinkActive,
            )
            runCatching { apiClient.patchMySellerProfile(request) }
                .onSuccess { profile ->
                    _state.update {
                        it.copy(
                            saving = false,
                            contactPhone = profile.contactPhone.orEmpty(),
                            operatingRegion = profile.operatingRegion.orEmpty(),
                            publicCode = profile.publicCode.orEmpty(),
                            shareLinkActive = profile.shareLinkActive,
                            snackbarCode = "SAVED",
                        )
                    }
                }
                .onFailure {
                    _state.update { it.copy(saving = false, snackbarCode = "SAVE_FAILED") }
                }
        }
    }
}
