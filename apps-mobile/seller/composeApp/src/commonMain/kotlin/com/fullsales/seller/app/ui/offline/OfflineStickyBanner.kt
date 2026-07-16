package com.fullsales.seller.app.ui.offline

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.offline.OfflineBannerReason
import com.fullsales.seller.shared.offline.OfflineBannerState

data class OfflineBannerUi(
    val state: OfflineBannerState = OfflineBannerState(
        visible = false,
        reason = OfflineBannerReason.None,
        pendingCount = 0,
    ),
    val onOpenHub: () -> Unit = {},
)

val LocalOfflineBannerUi = staticCompositionLocalOf { OfflineBannerUi() }

@Composable
fun OfflineStickyBanner(
    state: OfflineBannerState,
    onOpenHub: () -> Unit,
    modifier: Modifier = Modifier,
) {
    if (!state.visible) return
    val s = LocalSellerStrings.current
    val reason = when (state.reason) {
        OfflineBannerReason.Network -> s.offline.bannerNetwork
        OfflineBannerReason.Server -> s.offline.bannerServer
        OfflineBannerReason.None -> return
    }
    Surface(
        modifier = modifier
            .fillMaxWidth()
            .clickable(onClick = onOpenHub),
        color = MaterialTheme.colorScheme.primaryContainer,
        contentColor = MaterialTheme.colorScheme.onPrimaryContainer,
    ) {
        Column(
            modifier = Modifier.padding(horizontal = 16.dp, vertical = 10.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Text(s.offline.bannerTitle, style = MaterialTheme.typography.titleSmall)
            Text(reason, style = MaterialTheme.typography.bodySmall)
            if (state.showPendingChip) {
                Text(
                    s.offline.bannerPending.replace("{n}", state.pendingCount.toString()),
                    style = MaterialTheme.typography.labelMedium,
                    modifier = Modifier
                        .background(
                            MaterialTheme.colorScheme.secondaryContainer,
                            MaterialTheme.shapes.small,
                        )
                        .padding(horizontal = 8.dp, vertical = 2.dp),
                )
            }
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.End,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    s.offline.hubTitle,
                    style = MaterialTheme.typography.labelLarge,
                )
            }
        }
    }
}
