package com.fullsales.seller.android.ui.shell

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
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.fullsales.seller.android.ui.SellerRoutes
import com.fullsales.seller.android.ui.sync.SyncBadge

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SellerShellScaffold(
    currentRoute: String?,
    displayName: String?,
    logoUrl: String?,
    syncBadge: SyncBadge,
    onNavigateSales: () -> Unit,
    onNavigateNewSale: () -> Unit,
    onLogout: () -> Unit,
    content: @Composable (PaddingValues) -> Unit,
) {
    val showBottomBar = SellerRoutes.showsBottomBar(currentRoute)
    var menuExpanded by remember { mutableStateOf(false) }
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        if (!logoUrl.isNullOrBlank()) {
                            AsyncImage(
                                model = logoUrl,
                                contentDescription = displayName,
                                modifier = Modifier
                                    .size(32.dp)
                                    .padding(end = 8.dp),
                                contentScale = ContentScale.Fit,
                            )
                        }
                        Text(displayName ?: "Seller")
                    }
                },
                actions = {
                    SyncBadgeChip(syncBadge)
                    IconButton(onClick = { menuExpanded = true }) {
                        Icon(Icons.Default.MoreVert, contentDescription = "Menu")
                    }
                    DropdownMenu(expanded = menuExpanded, onDismissRequest = { menuExpanded = false }) {
                        DropdownMenuItem(
                            text = { Text("Logout") },
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
                        icon = { Icon(Icons.Default.Receipt, contentDescription = "Sales") },
                        label = { Text("Sales") },
                    )
                    NavigationBarItem(
                        selected = currentRoute == SellerRoutes.SALES_NEW,
                        onClick = onNavigateNewSale,
                        icon = { Icon(Icons.Default.Add, contentDescription = "New sale") },
                        label = { Text("New sale") },
                    )
                }
            }
        },
        content = content,
    )
}

@Composable
private fun SyncBadgeChip(badge: SyncBadge) {
    val label = when (badge) {
        SyncBadge.Offline -> "Offline"
        SyncBadge.Syncing -> "Syncing"
        SyncBadge.SyncFailed -> "Sync failed"
        SyncBadge.Idle -> return
    }
    Box(modifier = Modifier.padding(end = 4.dp)) {
        Text(
            text = label,
            style = MaterialTheme.typography.labelSmall,
            color = when (badge) {
                SyncBadge.SyncFailed -> MaterialTheme.colorScheme.error
                SyncBadge.Offline -> MaterialTheme.colorScheme.outline
                else -> MaterialTheme.colorScheme.primary
            },
            modifier = Modifier.padding(horizontal = 8.dp),
        )
    }
}
