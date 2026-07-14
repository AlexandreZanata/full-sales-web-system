package com.fullsales.seller.app.ui.shell

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.AssistChip
import androidx.compose.material3.AssistChipDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.LiveRegionMode
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.liveRegion
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.sync.SyncBadge
import com.fullsales.seller.app.ui.sync.shouldShowInHeader

@Composable
fun SyncBadgeChip(
    badge: SyncBadge,
    onRefresh: (() -> Unit)? = null,
) {
    if (!badge.shouldShowInHeader()) return
    val s = LocalSellerStrings.current
    val (label, colors) = when (badge) {
        SyncBadge.Offline -> s.common.offline to AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.error,
            labelColor = MaterialTheme.colorScheme.onError,
        )
        SyncBadge.Syncing -> s.common.syncing to AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.primaryContainer,
            labelColor = MaterialTheme.colorScheme.onPrimaryContainer,
        )
        SyncBadge.SyncFailed -> s.common.syncFailed to AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.errorContainer,
            labelColor = MaterialTheme.colorScheme.onErrorContainer,
        )
        SyncBadge.Online, SyncBadge.Connecting -> return
    }
    var announce by remember { mutableStateOf(label) }
    LaunchedEffect(badge) { announce = label }
    Box(
        modifier = Modifier
            .padding(end = 4.dp)
            .semantics(mergeDescendants = true) {
                contentDescription = announce
                liveRegion = LiveRegionMode.Polite
            },
    ) {
        AssistChip(
            onClick = { onRefresh?.invoke() },
            enabled = onRefresh != null && badge != SyncBadge.Syncing,
            label = { Text(label, style = MaterialTheme.typography.labelSmall) },
            colors = colors,
        )
    }
}
