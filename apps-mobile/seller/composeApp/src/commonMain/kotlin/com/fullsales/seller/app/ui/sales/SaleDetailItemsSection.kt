package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.theme.SellerWarningColor
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.formatMoneyMinorUnits
import com.fullsales.seller.shared.sales.SaleDetailLine
import com.fullsales.seller.shared.sales.saleLineNeedsBackorderWarning

@Composable
internal fun SaleDetailItemsCard(
    items: List<SaleDetailLine>,
    stockByProductId: Map<String, Int>,
    showBackorderHints: Boolean,
    mediaUrlResolver: MediaUrlResolver,
) {
    val s = LocalSellerStrings.current
    val itemCount = items.sumOf { it.quantity }
    Card(
        shape = MaterialTheme.shapes.large,
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surface),
        border = BorderStroke(1.dp, MaterialTheme.colorScheme.outlineVariant),
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    s.sales.productList,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                )
                Text(
                    SellerStrings.format(s.sales.itemsCountLabel, "count" to itemCount.toString()),
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Column(modifier = Modifier.padding(top = 12.dp)) {
                items.forEachIndexed { index, line ->
                    if (index > 0) {
                        HorizontalDivider(
                            modifier = Modifier.padding(vertical = 12.dp),
                            color = MaterialTheme.colorScheme.outlineVariant,
                        )
                    }
                    SaleDetailItemRow(
                        line = line,
                        isBackorder = showBackorderHints &&
                            saleLineNeedsBackorderWarning(line.productId, line.quantity, stockByProductId),
                        mediaUrlResolver = mediaUrlResolver,
                    )
                }
            }
        }
    }
}

@Composable
private fun SaleDetailItemRow(
    line: SaleDetailLine,
    isBackorder: Boolean,
    mediaUrlResolver: MediaUrlResolver,
) {
    val s = LocalSellerStrings.current
    var imageUrl by remember(line.productId) { mutableStateOf(line.primaryImageUrl) }
    LaunchedEffect(line.productId, line.primaryImageUrl, line.primaryImageFileId) {
        imageUrl = mediaUrlResolver.resolveImageUrl(line.primaryImageUrl, line.primaryImageFileId)
    }
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.Top,
    ) {
        ProductLineThumbnail(imageUrl = imageUrl, contentDescription = line.productName)
        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            Text(
                line.productName,
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            line.productSku?.let { sku ->
                Surface(shape = MaterialTheme.shapes.small, color = MaterialTheme.colorScheme.primaryContainer) {
                    Text(
                        sku,
                        modifier = Modifier.padding(horizontal = 8.dp, vertical = 2.dp),
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onPrimaryContainer,
                    )
                }
            }
            if (isBackorder) {
                BackorderBadge()
            }
            Text(
                "× ${line.quantity}",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            if (isBackorder) {
                Text(
                    SellerStrings.stockBackorderWarning(s),
                    color = SellerWarningColor,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
        }
        Text(
            formatMoneyMinorUnits(line.lineTotalMinor.toLong(), line.currency),
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.SemiBold,
            color = MaterialTheme.colorScheme.primary,
        )
    }
}
