package com.fullsales.seller.app.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Shapes
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
    secondaryContainer = Color(0xFFCCE8E4),
    onSecondaryContainer = Color(0xFF05201C),
    background = Color(0xFFFAFDFC),
    onBackground = Color(0xFF191C1C),
    surface = Color(0xFFFAFDFC),
    onSurface = Color(0xFF191C1C),
    surfaceVariant = Color(0xFFDAE5E3),
    onSurfaceVariant = Color(0xFF3F4948),
    outline = Color(0xFF6F7978),
    outlineVariant = Color(0xFFBEC9C7),
    error = SellerError,
    onError = SellerOnError,
    errorContainer = Color(0xFFFFDAD6),
    onErrorContainer = Color(0xFF410002),
)

private val SellerDarkColors = darkColorScheme(
    primary = SellerPrimaryContainer,
    onPrimary = SellerOnPrimaryContainer,
    primaryContainer = SellerPrimary,
    onPrimaryContainer = SellerOnPrimaryContainer,
    secondary = Color(0xFFB0CCC7),
    onSecondary = Color(0xFF1B3531),
    secondaryContainer = Color(0xFF324B47),
    onSecondaryContainer = Color(0xFFCCE8E4),
    background = Color(0xFF0F1413),
    onBackground = Color(0xFFE0E3E2),
    surface = Color(0xFF0F1413),
    onSurface = Color(0xFFE0E3E2),
    surfaceVariant = Color(0xFF3F4948),
    onSurfaceVariant = Color(0xFFBEC9C7),
    outline = Color(0xFF899392),
    outlineVariant = Color(0xFF3F4948),
    error = Color(0xFFFFB4AB),
    onError = Color(0xFF690005),
    errorContainer = Color(0xFF93000A),
    onErrorContainer = Color(0xFFFFDAD6),
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
    val dynamic = sellerDynamicColorScheme(darkTheme)
    val colorScheme = dynamic ?: if (darkTheme) SellerDarkColors else SellerLightColors
    SellerSystemBarsEffect(darkTheme)
    CompositionLocalProvider(LocalDensity provides scaledDensity) {
        MaterialTheme(
            colorScheme = colorScheme,
            typography = Typography(),
            shapes = Shapes(),
            content = content,
        )
    }
}
