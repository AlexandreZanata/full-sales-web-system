package com.fullsales.seller.app.ui.sales

import com.fullsales.seller.app.platform.CreateSaleDraftStore
import com.fullsales.seller.shared.sales.CreateSaleLineInput
import com.fullsales.seller.shared.sales.createSaleDraftFrom
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

internal fun restoreCreateSaleDraft(
    draftStore: CreateSaleDraftStore,
    state: MutableStateFlow<CreateSaleUiState>,
    onProductIds: (List<String>) -> Unit,
) {
    val draft = draftStore.read() ?: return
    state.update { current ->
        current.copy(
            commerceId = draft.commerceId,
            paymentMethod = draft.paymentMethod,
            lines = draft.lines.ifEmpty { listOf(CreateSaleLineInput()) },
        )
    }
    onProductIds(draft.lines.map { it.productId }.filter { it.isNotBlank() })
}

internal fun CoroutineScope.observeCreateSaleDraftPersistence(
    draftStore: CreateSaleDraftStore,
    state: MutableStateFlow<CreateSaleUiState>,
) {
    launch {
        state
            .map { createSaleDraftFrom(it.commerceId, it.paymentMethod, it.lines) }
            .distinctUntilChanged()
            .collect { draft ->
                if (draft.isEffectivelyEmpty()) draftStore.clear() else draftStore.write(draft)
            }
    }
}
