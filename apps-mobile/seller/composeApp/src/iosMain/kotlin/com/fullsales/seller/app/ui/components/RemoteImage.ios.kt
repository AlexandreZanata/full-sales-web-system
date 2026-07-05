package com.fullsales.seller.app.ui.components

import androidx.compose.foundation.layout.size
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
actual fun RemoteImage(
    url: String?,
    contentDescription: String?,
    modifier: Modifier,
) {
    // ponytail: Coil/Kamel image loading deferred; placeholder keeps iOS shell unblocked.
    Surface(modifier = modifier, color = MaterialTheme.colorScheme.surfaceVariant) {}
}
