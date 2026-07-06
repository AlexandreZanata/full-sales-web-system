package com.fullsales.seller.shared.a11y

enum class TextSizePreset(val scaleFactor: Float) {
    Normal(1.0f),
    Large(1.15f),
    ExtraLarge(1.3f),
    ;

    companion object {
        fun fromTag(tag: String?): TextSizePreset =
            entries.firstOrNull { it.name == tag } ?: Normal
    }
}

fun effectiveFontScale(systemFontScale: Float, preset: TextSizePreset): Float =
    systemFontScale * preset.scaleFactor
