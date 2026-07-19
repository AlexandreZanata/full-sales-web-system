package com.fullsales.seller.app.ui.offline

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material.icons.filled.Sync
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.components.SellerPrimaryButton
import com.fullsales.seller.app.ui.components.SellerSecondaryButton
import com.fullsales.seller.app.ui.components.SellerSectionTitle
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
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
        Text(
            s.offline.hubTitle,
            style = MaterialTheme.typography.headlineSmall,
            fontWeight = FontWeight.Bold,
            modifier = Modifier.screenTitle(),
        )
        SellerSurfaceCard(highlighted = true, contentPadding = false) {
            Column(
                modifier = Modifier.padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                Text(
                    statusLabel(state.statusReason, s.offline.statusOnline, s.offline.statusOffline, s.offline.statusServer),
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                    color = MaterialTheme.colorScheme.onPrimaryContainer,
                )
                SyncStamp(s.offline.hubLastCatalog, state.lastCatalogEpochMs, s.offline.hubNeverSynced)
                SyncStamp(s.offline.hubLastSales, state.lastSalesEpochMs, s.offline.hubNeverSynced)
                SyncStamp(s.offline.hubLastRegistrations, state.lastRegistrationsEpochMs, s.offline.hubNeverSynced)
            }
        }
        if (state.emptyCache) {
            Text(s.offline.hubEmptyCache, style = MaterialTheme.typography.bodyMedium)
        }
        SellerSectionTitle(s.offline.hubPendingTitle)
        SellerSurfaceCard(contentPadding = false) {
            Column(
                modifier = Modifier.padding(14.dp),
                verticalArrangement = Arrangement.spacedBy(6.dp),
            ) {
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
            }
        }
        Text(s.offline.hubWorksOffline, style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
        Text(s.offline.hubNeedsInternet, style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
        if (state.syncMessage == "STILL_OFFLINE") {
            Text(s.offline.bannerNetwork, color = MaterialTheme.colorScheme.error)
        }
        SellerPrimaryButton(
            onClick = viewModel::trySyncNow,
            leadingIcon = Icons.Default.Sync,
        ) { Text(s.offline.hubTrySync) }
        SellerSecondaryButton(
            onClick = {
                viewModel.clearMessage()
                onContinueOffline()
            },
            leadingIcon = Icons.AutoMirrored.Filled.ArrowForward,
        ) { Text(s.offline.hubContinue) }
    }
}

@Composable
private fun SyncStamp(label: String, epochMs: Long?, never: String) {
    Text(
        "$label: ${formatSyncEpochIso(epochMs) ?: never}",
        style = MaterialTheme.typography.bodyMedium,
        color = MaterialTheme.colorScheme.onPrimaryContainer,
    )
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
