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
    error = SellerError,
    onError = SellerOnError,
)

private val SellerDarkColors = darkColorScheme(
    primary = SellerPrimaryContainer,
    onPrimary = SellerOnPrimaryContainer,
    primaryContainer = SellerPrimary,
    onPrimaryContainer = SellerPrimaryContainer,
    secondary = SellerSecondary,
    error = SellerError,
)

@Composable
expect fun sellerDynamicColorScheme(darkTheme: Boolean): ColorScheme?

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
    CompositionLocalProvider(LocalDensity provides scaledDensity) {
        MaterialTheme(
            colorScheme = colorScheme,
            typography = Typography(),
            shapes = Shapes(),
            content = content,
        )
    }
}
