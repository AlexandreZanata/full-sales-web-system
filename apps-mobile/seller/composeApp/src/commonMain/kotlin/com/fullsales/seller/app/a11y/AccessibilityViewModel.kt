package com.fullsales.seller.app.a11y

import androidx.lifecycle.ViewModel
import com.fullsales.seller.app.platform.AccessibilityStore
import com.fullsales.seller.shared.a11y.TextSizePreset
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

class AccessibilityViewModel(
    private val store: AccessibilityStore = AccessibilityStore(),
) : ViewModel() {
    private val _preset = MutableStateFlow(store.read())
    val preset: StateFlow<TextSizePreset> = _preset.asStateFlow()

    fun setPreset(preset: TextSizePreset) {
        if (_preset.value == preset) return
        store.write(preset)
        _preset.value = preset
    }
}
