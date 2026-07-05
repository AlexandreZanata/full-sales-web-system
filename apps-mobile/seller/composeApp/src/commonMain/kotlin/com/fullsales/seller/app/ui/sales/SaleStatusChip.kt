package com.fullsales.seller.app.ui.sales

import androidx.compose.material3.AssistChip
import androidx.compose.material3.AssistChipDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.SaleDisplayStatus

@Composable
fun SaleStatusChip(status: SaleDisplayStatus, modifier: Modifier = Modifier) {
    val s = LocalSellerStrings.current
    val label = SellerStrings.saleStatus(s, status)
    val colors = when (status) {
        SaleDisplayStatus.PendingSync -> AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.primaryContainer,
            labelColor = MaterialTheme.colorScheme.onPrimaryContainer,
        )
        SaleDisplayStatus.SyncFailed -> AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.errorContainer,
            labelColor = MaterialTheme.colorScheme.onErrorContainer,
        )
        SaleDisplayStatus.Pending -> AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.surface,
            labelColor = MaterialTheme.colorScheme.onSurface,
        )
        SaleDisplayStatus.Cancelled -> AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant,
            labelColor = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        else -> AssistChipDefaults.assistChipColors()
    }
    AssistChip(
        onClick = {},
        enabled = false,
        label = { Text(label, style = MaterialTheme.typography.labelSmall) },
        colors = colors,
        modifier = modifier,
    )
}
