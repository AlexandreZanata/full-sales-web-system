package com.fullsales.seller.app.ui.components

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

@Composable
expect fun RemoteImage(
    url: String?,
    contentDescription: String?,
    modifier: Modifier = Modifier,
)
