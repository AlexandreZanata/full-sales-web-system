package com.fullsales.seller.app.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Typography
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.Density
import com.fullsales.seller.shared.a11y.TextSizePreset
import com.fullsales.seller.shared.a11y.effectiveFontScale

private val SellerLightColors = lightColorScheme(
    primary = SellerPrimary,
    onPrimary = SellerOnPrimary,
    primaryContainer = SellerPrimaryContainer,
    onPrimaryContainer = SellerOnPrimaryContainer,
    secondary = SellerSecondary,
    onSecondary = SellerOnSecondary,
    secondaryContainer = Color(0xFFE2E8F0),
    onSecondaryContainer = Color(0xFF0F172A),
    background = Color(0xFFF8FAFC),
    onBackground = Color(0xFF0F172A),
    surface = Color(0xFFFFFFFF),
    onSurface = Color(0xFF0F172A),
    surfaceVariant = Color(0xFFE2E8F0),
    onSurfaceVariant = Color(0xFF475569),
    outline = Color(0xFF94A3B8),
    outlineVariant = Color(0xFFCBD5E1),
    error = SellerError,
    onError = SellerOnError,
    errorContainer = Color(0xFFFFEBEE),
    onErrorContainer = Color(0xFFB71C1C),
)

private val SellerDarkColors = darkColorScheme(
    primary = Color(0xFF4C9AFF),
    onPrimary = Color(0xFF002F6C),
    primaryContainer = Color(0xFF0B5BD3),
    onPrimaryContainer = Color(0xFFE7F0FF),
    secondary = Color(0xFFCBD5E1),
    onSecondary = Color(0xFF1E293B),
    secondaryContainer = Color(0xFF334155),
    onSecondaryContainer = Color(0xFFE2E8F0),
    background = Color(0xFF0F172A),
    onBackground = Color(0xFFE2E8F0),
    surface = Color(0xFF1E293B),
    onSurface = Color(0xFFE2E8F0),
    surfaceVariant = Color(0xFF334155),
    onSurfaceVariant = Color(0xFFCBD5E1),
    outline = Color(0xFF64748B),
    outlineVariant = Color(0xFF475569),
    error = Color(0xFFFF6B6B),
    onError = Color(0xFF3B0A0A),
    errorContainer = Color(0xFFB71C1C),
    onErrorContainer = Color(0xFFFFEBEE),
)

@Composable
expect fun sellerDynamicColorScheme(darkTheme: Boolean): ColorScheme?

@Composable
expect fun SellerSystemBarsEffect(darkTheme: Boolean)

@Composable
fun SellerTheme(
    textSizePreset: TextSizePreset = TextSizePreset.Normal,
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit,
) {
    val systemDensity = LocalDensity.current
    val scaledDensity = Density(
        density = systemDensity.density,
        fontScale = effectiveFontScale(systemDensity.fontScale, textSizePreset),
    )
    val colorScheme = sellerDynamicColorScheme(darkTheme)
        ?: if (darkTheme) SellerDarkColors else SellerLightColors
    SellerSystemBarsEffect(darkTheme)
    CompositionLocalProvider(LocalDensity provides scaledDensity) {
        MaterialTheme(
            colorScheme = colorScheme,
            typography = Typography(),
            shapes = SellerShapes,
            content = content,
        )
    }
}
