package com.fullsales.seller.app.ui.shell

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.navigationBars
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.statusBars
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
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
import com.fullsales.seller.app.ui.offline.LocalOfflineBannerUi
import com.fullsales.seller.app.ui.offline.OfflineStickyBanner
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
    onProfile: () -> Unit,
    onLogout: () -> Unit,
    onSyncRefresh: (() -> Unit)? = null,
    content: @Composable (PaddingValues) -> Unit,
) {
    val s = LocalSellerStrings.current
    val locale by localeViewModel.locale.collectAsState()
    val textSizePreset by accessibilityViewModel.preset.collectAsState()
    val showBottomBar = SellerRoutes.showsBottomBar(currentRoute)
    val offlineBanner = LocalOfflineBannerUi.current
    // Bottom nav already consumes nav-bar insets; without it, pad content so snackbars clear the system bar.
    val contentInsets =
        if (showBottomBar) NestedScreenWindowInsets else WindowInsets.navigationBars
    Scaffold(
        contentWindowInsets = contentInsets,
        topBar = {
            Column {
                TopAppBar(
                    windowInsets = WindowInsets.statusBars,
                    title = { Text(s.nav.sellerFallback) },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.surface,
                        titleContentColor = MaterialTheme.colorScheme.onSurface,
                        actionIconContentColor = MaterialTheme.colorScheme.primary,
                    ),
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
                            onProfile = onProfile,
                            onLogout = onLogout,
                        )
                    },
                )
                OfflineStickyBanner(
                    state = offlineBanner.state,
                    onOpenHub = offlineBanner.onOpenHub,
                )
            }
        },
        bottomBar = {
            if (showBottomBar) {
                SellerBottomNav(
                    currentRoute = currentRoute,
                    onNavigateSales = onNavigateSales,
                    onNavigateNewSale = onNavigateNewSale,
                    onNavigateCommerces = onNavigateCommerces,
                )
            }
        },
        content = content,
    )
}
