package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AssistChip
import androidx.compose.material3.AssistChipDefaults
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.SellerEmptyState
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.i18n.SyncChipStatus
import com.fullsales.seller.shared.model.formatMoneyMinorUnits
import com.fullsales.seller.shared.sales.SaleDetailModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SaleDetailScreen(
    saleId: String,
    viewModel: SaleDetailViewModel,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(saleId) { viewModel.load(saleId) }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            snackbarHostState.showSnackbar(SellerStrings.saleActionError(s, code))
            viewModel.clearSnackbar()
        }
    }
    Scaffold(snackbarHost = { SnackbarHost(snackbarHostState) }) { padding ->
        when {
            state.loading -> CircularProgressIndicator(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding)
                    .padding(24.dp),
            )
            state.errorCode != null -> SellerEmptyState(
                title = s.sales.loadErrorTitle,
                message = SellerStrings.saleActionError(s, state.errorCode!!),
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding),
            )
            state.detail != null -> SaleDetailContent(
                detail = state.detail!!,
                acting = state.acting,
                onConfirm = viewModel::confirm,
                onCancel = viewModel::cancel,
                modifier = Modifier.padding(padding),
            )
        }
    }
}

@Composable
private fun SaleDetailContent(
    detail: SaleDetailModel,
    acting: Boolean,
    onConfirm: () -> Unit,
    onCancel: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(s.sales.detail, style = MaterialTheme.typography.headlineSmall)
            SaleStatusChip(status = detail.status)
        }
        detail.syncChip?.let { SyncStatusChip(it) }
        Text(
            detail.commerceName ?: detail.commerceId.take(8),
            style = MaterialTheme.typography.titleMedium,
        )
        Text(
            SellerStrings.format(
                s.sales.paymentLabel,
                "method" to SellerStrings.paymentMethod(s, detail.paymentMethod),
            ),
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(
            formatMoneyMinorUnits(detail.totalAmountMinor.toLong(), detail.totalCurrency),
            style = MaterialTheme.typography.headlineMedium,
        )
        Card(shape = MaterialTheme.shapes.medium) {
            Column {
                detail.items.forEach { line ->
                    ListItem(
                        headlineContent = { Text("${line.quantity}× ${line.productLabel}") },
                        supportingContent = { Text(line.productId.take(8)) },
                        trailingContent = {
                            Text(formatMoneyMinorUnits(line.lineTotalMinor.toLong(), line.currency))
                        },
                    )
                }
            }
        }
        if (detail.showActions) {
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                Button(
                    onClick = onConfirm,
                    enabled = !acting,
                    modifier = Modifier.weight(1f),
                ) {
                    if (acting) CircularProgressIndicator(modifier = Modifier.padding(4.dp))
                    else Text(s.sales.confirmShort)
                }
                OutlinedButton(
                    onClick = onCancel,
                    enabled = !acting,
                    modifier = Modifier.weight(1f),
                    colors = androidx.compose.material3.ButtonDefaults.outlinedButtonColors(
                        contentColor = MaterialTheme.colorScheme.error,
                    ),
                ) {
                    Text(s.sales.cancelShort)
                }
            }
        }
    }
}

@Composable
private fun SyncStatusChip(status: SyncChipStatus) {
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
