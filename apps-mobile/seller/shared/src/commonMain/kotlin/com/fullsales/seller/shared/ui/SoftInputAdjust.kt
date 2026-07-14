package com.fullsales.seller.shared.ui

/**
 * Soft-input adjust flags used by the seller Android host.
 * Contract: create-sale search fields require ADJUST_RESIZE so Compose IME insets resize the window.
 *
 * Values match [android.view.WindowManager.LayoutParams] SOFT_INPUT_ADJUST_* constants.
 */
object SoftInputAdjust {
    const val UNSPECIFIED: Int = 0x00
    const val RESIZE: Int = 0x10
    const val PAN: Int = 0x20
    const val NOTHING: Int = 0x30
    const val MASK: Int = 0xF0
}

fun softInputAdjustMode(softInputMode: Int): Int = softInputMode and SoftInputAdjust.MASK

fun requiresAdjustResizeForImeForms(softInputMode: Int): Boolean =
    softInputAdjustMode(softInputMode) == SoftInputAdjust.RESIZE
