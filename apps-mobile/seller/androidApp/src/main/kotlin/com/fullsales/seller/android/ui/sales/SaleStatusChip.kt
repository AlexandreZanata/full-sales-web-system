package com.fullsales.seller.android.ui.sales

import androidx.compose.material3.AssistChip
import androidx.compose.material3.AssistChipDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.fullsales.seller.shared.model.SaleDisplayStatus

@Composable
fun SaleStatusChip(status: SaleDisplayStatus, modifier: Modifier = Modifier) {
    val label = when (status) {
        SaleDisplayStatus.Pending -> "Pending"
        SaleDisplayStatus.Confirmed -> "Confirmed"
        SaleDisplayStatus.Cancelled -> "Cancelled"
        SaleDisplayStatus.PendingSync -> "Pending sync"
        SaleDisplayStatus.SyncFailed -> "Sync failed"
    }
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
