package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Payments
import androidx.compose.material.icons.filled.Store
import androidx.compose.material3.AssistChip
import androidx.compose.material3.AssistChipDefaults
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.SellerHighlightCard
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.i18n.SyncChipStatus
import com.fullsales.seller.shared.model.formatMoneyMinorUnits
import com.fullsales.seller.shared.sales.SaleDetailModel

@Composable
internal fun SaleDetailHeader(
    detail: SaleDetailModel,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(4.dp)) {
        Text(
            s.sales.detail,
            style = MaterialTheme.typography.headlineSmall,
            fontWeight = FontWeight.Bold,
        )
        Text(
            s.sales.detailSubtitle,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(
            SellerStrings.format(s.sales.saleIdLabel, "id" to detail.displayCode),
            style = MaterialTheme.typography.labelLarge,
            color = MaterialTheme.colorScheme.primary,
        )
    }
}

@Composable
internal fun SaleDetailSummaryCard(detail: SaleDetailModel) {
    val s = LocalSellerStrings.current
    SellerHighlightCard(contentPadding = false) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(20.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text(
                s.sales.orderSummary,
                style = MaterialTheme.typography.labelLarge,
                color = MaterialTheme.colorScheme.onPrimaryContainer,
            )
            Text(
                formatMoneyMinorUnits(detail.totalAmountMinor.toLong(), detail.totalCurrency),
                style = MaterialTheme.typography.displaySmall,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.onPrimaryContainer,
            )
            Row(
                horizontalArrangement = Arrangement.spacedBy(8.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                SaleStatusChip(status = detail.status)
                detail.syncChip?.let { SaleDetailSyncChip(it) }
            }
        }
    }
}

@Composable
internal fun SaleDetailMetaCard(detail: SaleDetailModel) {
    val s = LocalSellerStrings.current
    SellerSurfaceCard(contentPadding = false) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp),
        ) {
            SaleDetailMetaRow(
                icon = { Icon(Icons.Default.Store, contentDescription = null) },
                label = s.sales.commerce,
                value = detail.commerceName ?: detail.commerceId.take(8),
            )
            HorizontalDivider(color = MaterialTheme.colorScheme.outlineVariant)
            SaleDetailMetaRow(
                icon = { Icon(Icons.Default.Payments, contentDescription = null) },
                label = s.sales.paymentMethod,
                value = SellerStrings.paymentMethod(s, detail.paymentMethod),
            )
        }
    }
}

@Composable
private fun SaleDetailMetaRow(
    icon: @Composable () -> Unit,
    label: String,
    value: String,
) {
    Row(
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Surface(
            shape = MaterialTheme.shapes.medium,
            color = MaterialTheme.colorScheme.surface,
            tonalElevation = 1.dp,
        ) {
            Row(
                modifier = Modifier.padding(10.dp),
                content = { icon() },
            )
        }
        Column(modifier = Modifier.weight(1f)) {
            Text(
                label,
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Text(
                value,
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
        }
    }
}

@Composable
internal fun SaleDetailSyncChip(status: SyncChipStatus) {
    val s = LocalSellerStrings.current
    val label = when (status) {
        SyncChipStatus.PendingSync -> s.syncStatus.pendingSync
        SyncChipStatus.SyncFailed -> s.syncStatus.syncFailed
    }
    AssistChip(
        onClick = {},
        enabled = false,
        label = { Text(label, style = MaterialTheme.typography.labelSmall) },
        colors = AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.secondaryContainer,
            labelColor = MaterialTheme.colorScheme.onSecondaryContainer,
        ),
    )
}
