package com.fullsales.seller.android.ui.theme

import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Shapes
import androidx.compose.material3.Typography
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext

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
fun SellerTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    dynamicColor: Boolean = true,
    content: @Composable () -> Unit,
) {
    val context = LocalContext.current
    val colorScheme = when {
        dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }
        darkTheme -> SellerDarkColors
        else -> SellerLightColors
    }
    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography(),
        shapes = Shapes(),
        content = content,
    )
}
