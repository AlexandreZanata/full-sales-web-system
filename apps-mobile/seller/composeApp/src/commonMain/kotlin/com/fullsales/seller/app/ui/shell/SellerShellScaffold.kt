package com.fullsales.seller.app.ui.shell

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.Receipt
import androidx.compose.material3.AssistChip
import androidx.compose.material3.AssistChipDefaults
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.SellerRoutes
import com.fullsales.seller.app.ui.components.RemoteImage
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.i18n.LocaleSwitcher
import com.fullsales.seller.app.ui.sync.SyncBadge

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SellerShellScaffold(
    currentRoute: String?,
    displayName: String?,
    logoUrl: String?,
    syncBadge: SyncBadge,
    localeViewModel: LocaleViewModel,
    onNavigateSales: () -> Unit,
    onNavigateNewSale: () -> Unit,
    onLogout: () -> Unit,
    content: @Composable (PaddingValues) -> Unit,
) {
    val s = LocalSellerStrings.current
    val locale by localeViewModel.locale.collectAsState()
    val showBottomBar = SellerRoutes.showsBottomBar(currentRoute)
    var menuExpanded by remember { mutableStateOf(false) }
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        if (!logoUrl.isNullOrBlank()) {
                            RemoteImage(
                                url = logoUrl,
                                contentDescription = displayName,
                                modifier = Modifier
                                    .size(32.dp)
                                    .padding(end = 8.dp),
                            )
                        }
                        Text(displayName ?: s.nav.sellerFallback)
                    }
                },
                actions = {
                    LocaleSwitcher(
                        locale = locale,
                        onLocaleChange = localeViewModel::setLocale,
                        modifier = Modifier.padding(end = 8.dp),
                    )
                    SyncBadgeChip(syncBadge)
                    IconButton(onClick = { menuExpanded = true }) {
                        Icon(Icons.Default.MoreVert, contentDescription = s.a11y.menu)
                    }
                    DropdownMenu(expanded = menuExpanded, onDismissRequest = { menuExpanded = false }) {
                        DropdownMenuItem(
                            text = { Text(s.nav.logout) },
                            onClick = {
                                menuExpanded = false
                                onLogout()
                            },
                        )
                    }
                },
            )
        },
        bottomBar = {
            if (showBottomBar) {
                NavigationBar {
                    NavigationBarItem(
                        selected = currentRoute == SellerRoutes.SALES,
                        onClick = onNavigateSales,
                        icon = { Icon(Icons.Default.Receipt, contentDescription = s.a11y.sales) },
                        label = { Text(s.nav.sales) },
                    )
                    NavigationBarItem(
                        selected = currentRoute == SellerRoutes.SALES_NEW,
                        onClick = onNavigateNewSale,
                        icon = { Icon(Icons.Default.Add, contentDescription = s.a11y.newSale) },
                        label = { Text(s.nav.newSale) },
                    )
                }
            }
        },
        content = content,
    )
}

@Composable
private fun SyncBadgeChip(badge: SyncBadge) {
    val s = LocalSellerStrings.current
    val (label, colors) = when (badge) {
        SyncBadge.Offline -> s.common.offline to AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant,
            labelColor = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        SyncBadge.Syncing -> s.common.syncing to AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.primaryContainer,
            labelColor = MaterialTheme.colorScheme.onPrimaryContainer,
        )
        SyncBadge.SyncFailed -> s.common.syncFailed to AssistChipDefaults.assistChipColors(
            containerColor = MaterialTheme.colorScheme.errorContainer,
            labelColor = MaterialTheme.colorScheme.onErrorContainer,
        )
        SyncBadge.Idle -> return
    }
    Box(modifier = Modifier.padding(end = 4.dp)) {
        AssistChip(
            onClick = {},
            enabled = false,
            label = { Text(label, style = MaterialTheme.typography.labelSmall) },
            colors = colors,
        )
    }
}
