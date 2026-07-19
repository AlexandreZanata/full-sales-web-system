package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Share
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings

private val CatalogShareBlue = Color(0xFF1565C0)

@Composable
fun CatalogShareBar(
    catalogUrl: String?,
    enabled: Boolean,
    onShare: () -> Unit,
    onCopy: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    Row(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .background(CatalogShareBlue)
            .padding(horizontal = 14.dp, vertical = 10.dp)
            .semantics { contentDescription = s.sales.catalogLinkTitle },
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(2.dp)) {
            Text(
                s.sales.catalogLinkTitle,
                color = Color.White,
                style = MaterialTheme.typography.titleSmall,
                fontWeight = FontWeight.SemiBold,
            )
            if (catalogUrl != null) {
                Text(
                    catalogUrl,
                    color = Color.White.copy(alpha = 0.85f),
                    style = MaterialTheme.typography.bodySmall,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                )
            } else {
                Text(
                    s.sales.catalogLinkUnavailable,
                    color = Color.White.copy(alpha = 0.85f),
                    style = MaterialTheme.typography.bodySmall,
                )
            }
        }
        IconButton(onClick = onShare, enabled = enabled) {
            Icon(
                Icons.Default.Share,
                contentDescription = s.sales.catalogLinkShare,
                tint = if (enabled) Color.White else Color.White.copy(alpha = 0.4f),
            )
        }
        IconButton(onClick = onCopy, enabled = enabled) {
            Icon(
                Icons.Default.ContentCopy,
                contentDescription = s.sales.catalogLinkCopy,
                tint = if (enabled) Color.White else Color.White.copy(alpha = 0.4f),
            )
        }
    }
}
