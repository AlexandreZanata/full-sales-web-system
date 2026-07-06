package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.a11y.TextSizePreset

expect class AccessibilityStore() {
    fun read(): TextSizePreset
    fun write(preset: TextSizePreset)
}
