package com.fullsales.seller.app.ui.offline

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.offline.OfflineBannerReason
import com.fullsales.seller.shared.offline.formatSyncEpochIso

@Composable
fun OfflineHubScreen(
    viewModel: OfflineHubViewModel,
    onContinueOffline: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text(s.offline.hubTitle, style = MaterialTheme.typography.headlineSmall, modifier = Modifier.screenTitle())
        Text(statusLabel(state.statusReason, s.offline.statusOnline, s.offline.statusOffline, s.offline.statusServer))
        SyncStamp(s.offline.hubLastCatalog, state.lastCatalogEpochMs, s.offline.hubNeverSynced)
        SyncStamp(s.offline.hubLastSales, state.lastSalesEpochMs, s.offline.hubNeverSynced)
        SyncStamp(s.offline.hubLastRegistrations, state.lastRegistrationsEpochMs, s.offline.hubNeverSynced)
        if (state.emptyCache) {
            Text(s.offline.hubEmptyCache, style = MaterialTheme.typography.bodyMedium)
        }
        Text(s.offline.hubPendingTitle, style = MaterialTheme.typography.titleMedium)
        if (state.pending.isEmpty()) {
            Text("—", style = MaterialTheme.typography.bodySmall)
        } else {
            state.pending.take(20).forEach { entry ->
                Text(
                    "${entry.entityType} · ${entry.aggregateId.take(8)} · ${formatSyncEpochIso(entry.createdAtEpochMs)}",
                    style = MaterialTheme.typography.bodySmall,
                )
            }
        }
        Text(s.offline.hubWorksOffline, style = MaterialTheme.typography.bodySmall)
        Text(s.offline.hubNeedsInternet, style = MaterialTheme.typography.bodySmall)
        if (state.syncMessage == "STILL_OFFLINE") {
            Text(s.offline.bannerNetwork, color = MaterialTheme.colorScheme.error)
        }
        Button(
            onClick = viewModel::trySyncNow,
            modifier = Modifier.fillMaxWidth(),
        ) { Text(s.offline.hubTrySync) }
        OutlinedButton(
            onClick = {
                viewModel.clearMessage()
                onContinueOffline()
            },
            modifier = Modifier.fillMaxWidth(),
        ) { Text(s.offline.hubContinue) }
    }
}

@Composable
private fun SyncStamp(label: String, epochMs: Long?, never: String) {
    Text("$label: ${formatSyncEpochIso(epochMs) ?: never}", style = MaterialTheme.typography.bodyMedium)
}

private fun statusLabel(
    reason: OfflineBannerReason,
    online: String,
    offline: String,
    server: String,
): String = when (reason) {
    OfflineBannerReason.None -> online
    OfflineBannerReason.Network -> offline
    OfflineBannerReason.Server -> server
}
