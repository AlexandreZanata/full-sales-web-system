package com.fullsales.seller.android

import android.content.ComponentName
import android.content.pm.PackageManager
import android.view.WindowManager
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.fullsales.seller.shared.ui.requiresAdjustResizeForImeForms
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Contract: MainActivity must use ADJUST_RESIZE so create-sale product search
 * can keep the focused field visible above the soft keyboard.
 */
@RunWith(AndroidJUnit4::class)
class SoftInputModeInstrumentedTest {
    @Test
    fun given_mainActivity_when_readingSoftInputMode_then_adjustResize() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val info = context.packageManager.getActivityInfo(
            ComponentName(context, MainActivity::class.java),
            PackageManager.GET_META_DATA,
        )
        val softInputMode = info.softInputMode and WindowManager.LayoutParams.SOFT_INPUT_MASK_ADJUST
        assertTrue(
            "Expected ADJUST_RESIZE for IME forms, got 0x${Integer.toHexString(softInputMode)}",
            requiresAdjustResizeForImeForms(info.softInputMode),
        )
    }
}
