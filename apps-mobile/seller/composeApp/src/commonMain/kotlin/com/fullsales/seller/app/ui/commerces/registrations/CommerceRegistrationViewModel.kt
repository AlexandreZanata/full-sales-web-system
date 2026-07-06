package com.fullsales.seller.app.ui.commerces.registrations

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.CommerceRegistrationDraftStore
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.CnpjLookupResult
import com.fullsales.seller.shared.model.RegistrationMode
import com.fullsales.seller.shared.registrations.CommerceRegistrationDraft
import com.fullsales.seller.shared.registrations.CommerceRegistrationFormErrors
import com.fullsales.seller.shared.registrations.buildSubmitRegistrationRequest
import com.fullsales.seller.shared.registrations.draftFromLookup
import com.fullsales.seller.shared.registrations.validateCommerceRegistrationForm
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

data class CommerceRegistrationUiState(
    val draft: CommerceRegistrationDraft = CommerceRegistrationDraft(),
    val submitting: Boolean = false,
    val errors: CommerceRegistrationFormErrors = CommerceRegistrationFormErrors(),
    val snackbarCode: String? = null,
    val cnpjReadOnly: Boolean = false,
) {
    val hasPersistedContent: Boolean get() = !draft.isEffectivelyEmpty()
}

class CommerceRegistrationViewModel(
    private val apiClient: SellerApiClient,
    private val networkMonitor: NetworkMonitor,
    private val draftStore: CommerceRegistrationDraftStore,
) : ViewModel() {
    private val json = Json { ignoreUnknownKeys = true }
    private val _state = MutableStateFlow(CommerceRegistrationUiState())
    val state: StateFlow<CommerceRegistrationUiState> = _state.asStateFlow()

    init {
        restoreDraft()
        observeDraftPersistence()
    }

    fun startManual() {
        val draft = CommerceRegistrationDraft(mode = RegistrationMode.MANUAL)
        draftStore.write(draft)
        _state.update {
            it.copy(
                draft = draft,
                cnpjReadOnly = false,
                errors = CommerceRegistrationFormErrors(),
            )
        }
    }

    fun applyLookupResult(result: CnpjLookupResult) {
        val snapshot = json.encodeToString(result)
        val draft = draftFromLookup(result, snapshot)
        draftStore.write(draft)
        _state.update {
            it.copy(
                draft = draft,
                cnpjReadOnly = true,
                errors = CommerceRegistrationFormErrors(),
            )
        }
    }

    fun updateDraft(transform: (CommerceRegistrationDraft) -> CommerceRegistrationDraft) {
        _state.update { current ->
            current.copy(draft = transform(current.draft), errors = CommerceRegistrationFormErrors())
        }
    }

    fun clearForm() {
        draftStore.clear()
        _state.value = CommerceRegistrationUiState()
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun submit() {
        val draft = _state.value.draft
        val errors = validateCommerceRegistrationForm(draft)
        if (!errors.isValid) {
            _state.update { it.copy(errors = errors) }
            return
        }
        if (!networkMonitor.isOnline()) {
            _state.update { it.copy(snackbarCode = "NETWORK_ERROR") }
            return
        }
        viewModelScope.launch {
            _state.update { it.copy(submitting = true, errors = CommerceRegistrationFormErrors()) }
            runCatching {
                apiClient.submitRegistration(buildSubmitRegistrationRequest(draft))
            }.onSuccess {
                draftStore.clear()
                _state.update { it.copy(submitting = false, snackbarCode = "SUBMITTED") }
            }.onFailure { err ->
                val code = (err as? ApiException)?.detail?.code ?: "SUBMIT_FAILED"
                _state.update { it.copy(submitting = false, snackbarCode = code) }
            }
        }
    }

    private fun restoreDraft() {
        val draft = draftStore.read() ?: return
        _state.update {
            it.copy(
                draft = draft,
                cnpjReadOnly = draft.mode == RegistrationMode.CNPJ_LOOKUP && draft.cnpj.isNotBlank(),
            )
        }
    }

    private fun observeDraftPersistence() {
        viewModelScope.launch {
            _state.map { it.draft }.distinctUntilChanged().collect { draft ->
                if (draft.isEffectivelyEmpty()) draftStore.clear() else draftStore.write(draft)
            }
        }
    }
}
