package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Inventory2
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.ui.components.RemoteImage
import com.fullsales.seller.shared.model.Product

@Composable
internal fun SaleLineProductImage(
    product: Product?,
    mediaUrlResolver: MediaUrlResolver,
    contentDescription: String,
    modifier: Modifier = Modifier,
) {
    var imageUrl by remember(product?.id) { mutableStateOf(product?.primaryImageUrl) }
    LaunchedEffect(product?.id, product?.primaryImageUrl, product?.primaryImageFileId) {
        if (product == null) {
            imageUrl = null
            return@LaunchedEffect
        }
        imageUrl = mediaUrlResolver.resolveImageUrl(product.primaryImageUrl, product.primaryImageFileId)
    }
    ProductLineThumbnail(
        imageUrl = imageUrl,
        contentDescription = contentDescription,
        modifier = modifier,
    )
}

@Composable
internal fun ProductLineThumbnail(
    imageUrl: String?,
    contentDescription: String,
    modifier: Modifier = Modifier,
) {
    Surface(
        modifier = modifier
            .size(72.dp)
            .clip(RoundedCornerShape(12.dp)),
        color = MaterialTheme.colorScheme.surfaceVariant,
        tonalElevation = 1.dp,
    ) {
        if (imageUrl.isNullOrBlank()) {
            Box(
                modifier = Modifier.size(72.dp),
                contentAlignment = Alignment.Center,
            ) {
                Icon(
                    imageVector = Icons.Outlined.Inventory2,
                    contentDescription = contentDescription,
                    modifier = Modifier.size(36.dp),
                    tint = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        } else {
            RemoteImage(
                url = imageUrl,
                contentDescription = contentDescription,
                modifier = Modifier.size(72.dp),
            )
        }
    }
}
