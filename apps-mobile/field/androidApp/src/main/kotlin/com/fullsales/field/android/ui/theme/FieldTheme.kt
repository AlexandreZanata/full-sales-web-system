package com.fullsales.field.android.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

private val LajantaBlue = Color(0xFF1565C0)
private val LajantaOnPrimary = Color(0xFFFFFFFF)
private val LajantaContainer = Color(0xFFD0E4FF)
private val LajantaOnContainer = Color(0xFF001D36)

private val FieldColors = lightColorScheme(
    primary = LajantaBlue,
    onPrimary = LajantaOnPrimary,
    primaryContainer = LajantaContainer,
    onPrimaryContainer = LajantaOnContainer,
)

@Composable
fun FieldTheme(content: @Composable () -> Unit) {
    MaterialTheme(colorScheme = FieldColors, content = content)
}
