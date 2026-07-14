package com.fullsales.seller.app.ui.shell

import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.navigationBars
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.statusBars
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Receipt
import androidx.compose.material.icons.filled.Store
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.SellerRoutes
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.i18n.LocaleSwitcher
import com.fullsales.seller.app.ui.sync.SyncBadge

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SellerShellScaffold(
    currentRoute: String?,
    syncBadge: SyncBadge,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
    onNavigateSales: () -> Unit,
    onNavigateNewSale: () -> Unit,
    onNavigateCommerces: () -> Unit,
    onLogout: () -> Unit,
    onSyncRefresh: (() -> Unit)? = null,
    content: @Composable (PaddingValues) -> Unit,
) {
    val s = LocalSellerStrings.current
    val locale by localeViewModel.locale.collectAsState()
    val textSizePreset by accessibilityViewModel.preset.collectAsState()
    val showBottomBar = SellerRoutes.showsBottomBar(currentRoute)
    Scaffold(
        contentWindowInsets = NestedScreenWindowInsets,
        topBar = {
            TopAppBar(
                windowInsets = WindowInsets.statusBars,
                title = { Text(s.nav.sellerFallback) },
                actions = {
                    LocaleSwitcher(
                        locale = locale,
                        onLocaleChange = localeViewModel::setLocale,
                        modifier = Modifier.padding(end = 4.dp),
                    )
                    SyncBadgeChip(syncBadge, onRefresh = onSyncRefresh)
                    ShellOverflowMenu(
                        textSizePreset = textSizePreset,
                        onTextSizeChange = accessibilityViewModel::setPreset,
                        onLogout = onLogout,
                    )
                },
            )
        },
        bottomBar = {
            if (showBottomBar) {
                NavigationBar(windowInsets = WindowInsets.navigationBars) {
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
                    NavigationBarItem(
                        selected = currentRoute == SellerRoutes.COMMERCES,
                        onClick = onNavigateCommerces,
                        icon = { Icon(Icons.Default.Store, contentDescription = s.a11y.commerces) },
                        label = { Text(s.nav.commerces) },
                    )
                }
            }
        },
        content = content,
    )
}
