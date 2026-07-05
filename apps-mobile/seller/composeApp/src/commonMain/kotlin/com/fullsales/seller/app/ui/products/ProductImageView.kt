package com.fullsales.seller.app.ui.products

import androidx.compose.foundation.layout.size
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.RemoteImage

@Composable
fun ProductThumbnail(
    imageUrl: String?,
    contentDescription: String,
    modifier: Modifier = Modifier,
) {
    ProductImage(imageUrl, contentDescription, modifier.size(48.dp), 48.dp)
}

@Composable
fun ProductHeroImage(
    imageUrl: String?,
    contentDescription: String,
    modifier: Modifier = Modifier,
) {
    ProductImage(imageUrl, contentDescription, modifier.size(160.dp), 160.dp)
}

@Composable
private fun ProductImage(
    imageUrl: String?,
    contentDescription: String,
    modifier: Modifier,
    placeholderSize: Dp,
) {
    if (imageUrl.isNullOrBlank()) {
        ProductImagePlaceholder(placeholderSize)
        return
    }
    RemoteImage(
        url = imageUrl,
        contentDescription = contentDescription,
        modifier = modifier,
    )
}

@Composable
private fun ProductImagePlaceholder(size: Dp) {
    Surface(
        modifier = Modifier.size(size),
        color = MaterialTheme.colorScheme.surfaceVariant,
    ) {}
}
