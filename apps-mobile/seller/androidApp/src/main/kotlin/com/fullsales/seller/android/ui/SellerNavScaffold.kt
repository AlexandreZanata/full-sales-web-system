package com.fullsales.seller.android.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import com.fullsales.seller.android.i18n.LocaleViewModel
import com.fullsales.seller.android.ui.auth.AuthViewModel
import com.fullsales.seller.android.ui.settings.SettingsUiState
import com.fullsales.seller.android.ui.settings.SettingsViewModel
import com.fullsales.seller.android.ui.shell.SellerShellScaffold
import com.fullsales.seller.android.ui.sync.SyncBadge

internal fun NavGraphBuilder.shellRoute(
    route: String,
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    settingsViewModel: SettingsViewModel,
    localeViewModel: LocaleViewModel,
    content: @Composable () -> Unit,
) {
    composable(route) {
        LaunchedEffect(Unit) { settingsViewModel.loadIfStale() }
        ShellScaffold(route, navController, settings, syncBadge, authViewModel, localeViewModel) {
            content()
        }
    }
}

internal fun NavGraphBuilder.detailRoute(
    route: String,
    argName: String,
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    content: @Composable (String) -> Unit,
) {
    composable(route, arguments = listOf(navArgument(argName) { type = NavType.StringType })) { entry ->
        val id = entry.arguments?.getString(argName).orEmpty()
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel) {
            content(id)
        }
    }
}

@Composable
internal fun ShellScaffold(
    route: String,
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    content: @Composable () -> Unit,
) {
    SellerShellScaffold(
        currentRoute = route,
        displayName = settings.displayName,
        logoUrl = settings.logoUrl,
        syncBadge = syncBadge,
        localeViewModel = localeViewModel,
        onNavigateSales = { navController.navigate(SellerRoutes.SALES) { launchSingleTop = true } },
        onNavigateNewSale = { navController.navigate(SellerRoutes.SALES_NEW) { launchSingleTop = true } },
        onLogout = { logout(authViewModel, navController) },
    ) { padding ->
        Box(Modifier.padding(padding)) { content() }
    }
}

@Composable
internal fun DetailShell(
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    content: @Composable () -> Unit,
) {
    SellerShellScaffold(
        currentRoute = null,
        displayName = settings.displayName,
        logoUrl = settings.logoUrl,
        syncBadge = syncBadge,
        localeViewModel = localeViewModel,
        onNavigateSales = { navController.navigate(SellerRoutes.SALES) { launchSingleTop = true } },
        onNavigateNewSale = { navController.navigate(SellerRoutes.SALES_NEW) { launchSingleTop = true } },
        onLogout = { logout(authViewModel, navController) },
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
