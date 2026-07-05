package com.fullsales.seller.app.ui.components

import androidx.compose.foundation.layout.size
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import coil.compose.SubcomposeAsyncImage

@Composable
actual fun RemoteImage(
    url: String?,
    contentDescription: String?,
    modifier: Modifier,
) {
    if (url.isNullOrBlank()) {
        Surface(modifier = modifier, color = MaterialTheme.colorScheme.surfaceVariant) {}
        return
    }
    SubcomposeAsyncImage(
        model = url,
        contentDescription = contentDescription,
        modifier = modifier,
        contentScale = ContentScale.Crop,
        loading = { Surface(modifier = Modifier.size(48.dp), color = MaterialTheme.colorScheme.surfaceVariant) {} },
        error = { Surface(modifier = Modifier.size(48.dp), color = MaterialTheme.colorScheme.surfaceVariant) {} },
    )
}
