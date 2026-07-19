package com.fullsales.seller.app.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.auth.AuthViewModel
import com.fullsales.seller.app.ui.shell.SellerShellScaffold
import com.fullsales.seller.app.ui.sync.SyncBadge

internal fun NavGraphBuilder.shellRoute(
    route: String,
    navController: NavHostController,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
    onSyncRefresh: (() -> Unit)? = null,
    content: @Composable () -> Unit,
) {
    composable(route) {
        ShellScaffold(
            route,
            navController,
            syncBadge,
            authViewModel,
            localeViewModel,
            accessibilityViewModel,
            onSyncRefresh,
        ) {
            content()
        }
    }
}

internal fun NavGraphBuilder.detailRoute(
    route: String,
    argName: String,
    navController: NavHostController,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
    onSyncRefresh: (() -> Unit)? = null,
    content: @Composable (String) -> Unit,
) {
    composable(route, arguments = listOf(navArgument(argName) { type = NavType.StringType })) { entry ->
        val id = entry.arguments?.getString(argName).orEmpty()
        DetailShell(
            navController,
            syncBadge,
            authViewModel,
            localeViewModel,
            accessibilityViewModel,
            onSyncRefresh,
        ) {
            content(id)
        }
    }
}

@Composable
internal fun ShellScaffold(
    route: String,
    navController: NavHostController,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
    onSyncRefresh: (() -> Unit)? = null,
    content: @Composable () -> Unit,
) {
    SellerShellScaffold(
        currentRoute = route,
        syncBadge = syncBadge,
        localeViewModel = localeViewModel,
        accessibilityViewModel = accessibilityViewModel,
        onNavigateSales = { navController.navigate(SellerRoutes.SALES) { launchSingleTop = true } },
        onNavigateNewSale = { navController.navigate(SellerRoutes.SALES_NEW) { launchSingleTop = true } },
        onNavigateCommerces = { navController.navigate(SellerRoutes.COMMERCES) { launchSingleTop = true } },
        onProfile = { navController.navigate(SellerRoutes.PROFILE) { launchSingleTop = true } },
        onLogout = { logout(authViewModel, navController) },
        onSyncRefresh = onSyncRefresh,
    ) { padding ->
        Box(Modifier.padding(padding)) { content() }
    }
}

@Composable
internal fun DetailShell(
    navController: NavHostController,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
    onSyncRefresh: (() -> Unit)? = null,
    content: @Composable () -> Unit,
) {
    SellerShellScaffold(
        currentRoute = null,
        syncBadge = syncBadge,
        localeViewModel = localeViewModel,
        accessibilityViewModel = accessibilityViewModel,
        onNavigateSales = { navController.navigate(SellerRoutes.SALES) { launchSingleTop = true } },
        onNavigateNewSale = { navController.navigate(SellerRoutes.SALES_NEW) { launchSingleTop = true } },
        onNavigateCommerces = { navController.navigate(SellerRoutes.COMMERCES) { launchSingleTop = true } },
        onProfile = { navController.navigate(SellerRoutes.PROFILE) { launchSingleTop = true } },
        onLogout = { logout(authViewModel, navController) },
        onSyncRefresh = onSyncRefresh,
    ) { padding ->
        Box(Modifier.padding(padding)) { content() }
    }
}

internal fun logout(authViewModel: AuthViewModel, navController: NavHostController) {
    authViewModel.logout {
        navController.navigate(SellerRoutes.LOGIN) {
            popUpTo(0) { inclusive = true }
        }
    }
}
