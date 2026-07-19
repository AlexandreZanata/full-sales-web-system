package com.fullsales.seller.app.ui.theme

import android.app.Activity
import android.content.ContextWrapper
import androidx.compose.material3.ColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat

/** Brand palette is fixed (Facebook blue); do not use Material You dynamic colors. */
@Composable
actual fun sellerDynamicColorScheme(darkTheme: Boolean): ColorScheme? = null

@Composable
actual fun SellerSystemBarsEffect(darkTheme: Boolean) {
    val view = LocalView.current
    if (!view.isInEditMode) {
        SideEffect {
            val window = view.context.findActivity()?.window ?: return@SideEffect
            WindowCompat.getInsetsController(window, view).isAppearanceLightStatusBars = !darkTheme
        }
    }
}

private tailrec fun android.content.Context.findActivity(): Activity? = when (this) {
    is Activity -> this
    is ContextWrapper -> baseContext.findActivity()
    else -> null
}
