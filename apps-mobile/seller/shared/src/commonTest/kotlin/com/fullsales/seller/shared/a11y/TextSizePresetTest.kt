package com.fullsales.seller.shared.a11y

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class TextSizePresetTest {
    @Test
    fun presetScaleFactors() {
        assertEquals(1.0f, TextSizePreset.Normal.scaleFactor)
        assertEquals(1.15f, TextSizePreset.Large.scaleFactor)
        assertEquals(1.3f, TextSizePreset.ExtraLarge.scaleFactor)
    }

    @Test
    fun effectiveFontScaleMultipliesSystemAndPreset() {
        assertEquals(1.3f, effectiveFontScale(1.0f, TextSizePreset.ExtraLarge))
        assertTrue(kotlin.math.abs(effectiveFontScale(1.5f, TextSizePreset.Large) - 1.725f) < 0.001f)
    }

    @Test
    fun fromTagFallsBackToNormal() {
        assertEquals(TextSizePreset.Large, TextSizePreset.fromTag("Large"))
        assertEquals(TextSizePreset.Normal, TextSizePreset.fromTag("unknown"))
        assertEquals(TextSizePreset.Normal, TextSizePreset.fromTag(null))
    }
}
