package com.fullsales.seller.android.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.fullsales.seller.android.AppContainer
import com.fullsales.seller.android.ui.auth.AuthViewModel
import com.fullsales.seller.android.ui.auth.LoginScreen
import com.fullsales.seller.android.ui.sales.SalesListScreen
import com.fullsales.seller.android.ui.sales.SalesViewModel
import com.fullsales.seller.android.ui.settings.SettingsUiState
import com.fullsales.seller.android.ui.settings.SettingsViewModel
import com.fullsales.seller.android.ui.shell.SellerShellScaffold
import com.fullsales.seller.android.ui.sync.SyncBadge
import com.fullsales.seller.android.ui.sync.SyncStatusViewModel

@Composable
fun SellerNavHost(container: AppContainer) {
    val factory = SellerViewModelFactory(container)
    val authViewModel: AuthViewModel = viewModel(factory = factory)
    val settingsViewModel: SettingsViewModel = viewModel(factory = factory)
    val salesViewModel: SalesViewModel = viewModel(factory = factory)
    val syncViewModel: SyncStatusViewModel = viewModel(factory = factory)
    val auth by authViewModel.state.collectAsState()
    val settings by settingsViewModel.state.collectAsState()
    val syncBadge by syncViewModel.badge.collectAsState()
    val navController = rememberNavController()
    val startDestination = if (auth.isAuthenticated) SellerRoutes.SALES else SellerRoutes.LOGIN

    NavHost(navController = navController, startDestination = startDestination) {
        composable(SellerRoutes.LOGIN) {
            LoginScreen(
                viewModel = authViewModel,
                onLoggedIn = {
                    settingsViewModel.loadIfStale(force = true)
                    navController.navigate(SellerRoutes.SALES) {
                        popUpTo(SellerRoutes.LOGIN) { inclusive = true }
                    }
                },
            )
        }
        shellRoute(SellerRoutes.SALES, navController, settings, syncBadge, authViewModel, settingsViewModel) {
            SalesListScreen(
                salesViewModel = salesViewModel,
                syncViewModel = syncViewModel,
                onSaleClick = { id -> navController.navigate(SellerRoutes.saleDetail(id)) },
            )
        }
        shellRoute(SellerRoutes.SALES_NEW, navController, settings, syncBadge, authViewModel, settingsViewModel) {
            PlaceholderScreen("New sale", "Create-sale flow — Phase 60")
        }
        detailRoute(SellerRoutes.SALE_DETAIL, "saleId", navController, settings, syncBadge, authViewModel) { id ->
            PlaceholderScreen("Sale $id", "Sale detail — Phase 61")
        }
        composable(SellerRoutes.COMMERCES) {
            DetailShell(navController, settings, syncBadge, authViewModel) {
                PlaceholderScreen("Commerces")
            }
        }
        detailRoute(SellerRoutes.COMMERCE_DETAIL, "commerceId", navController, settings, syncBadge, authViewModel) { id ->
            PlaceholderScreen("Commerce $id")
        }
        composable(SellerRoutes.PRODUCTS) {
            DetailShell(navController, settings, syncBadge, authViewModel) {
                PlaceholderScreen("Products")
            }
        }
        detailRoute(SellerRoutes.PRODUCT_DETAIL, "productId", navController, settings, syncBadge, authViewModel) { id ->
            PlaceholderScreen("Product $id")
        }
    }
}

private fun NavGraphBuilder.shellRoute(
    route: String,
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    settingsViewModel: SettingsViewModel,
    content: @Composable () -> Unit,
) {
    composable(route) {
        LaunchedEffect(Unit) { settingsViewModel.loadIfStale() }
        ShellScaffold(route, navController, settings, syncBadge, authViewModel) {
            content()
        }
    }
}

private fun NavGraphBuilder.detailRoute(
    route: String,
    argName: String,
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    content: @Composable (String) -> Unit,
) {
    composable(route, arguments = listOf(navArgument(argName) { type = NavType.StringType })) { entry ->
        val id = entry.arguments?.getString(argName).orEmpty()
        DetailShell(navController, settings, syncBadge, authViewModel) {
            content(id)
        }
    }
}

@Composable
private fun ShellScaffold(
    route: String,
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    content: @Composable () -> Unit,
) {
    SellerShellScaffold(
        currentRoute = route,
        displayName = settings.displayName,
        logoUrl = settings.logoUrl,
        syncBadge = syncBadge,
        onNavigateSales = { navController.navigate(SellerRoutes.SALES) { launchSingleTop = true } },
        onNavigateNewSale = { navController.navigate(SellerRoutes.SALES_NEW) { launchSingleTop = true } },
        onLogout = { logout(authViewModel, navController) },
    ) { padding ->
        androidx.compose.foundation.layout.Box(Modifier.padding(padding)) { content() }
    }
}

@Composable
private fun DetailShell(
    navController: NavHostController,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    content: @Composable () -> Unit,
) {
    SellerShellScaffold(
        currentRoute = null,
        displayName = settings.displayName,
        logoUrl = settings.logoUrl,
        syncBadge = syncBadge,
        onNavigateSales = { navController.navigate(SellerRoutes.SALES) { launchSingleTop = true } },
        onNavigateNewSale = { navController.navigate(SellerRoutes.SALES_NEW) { launchSingleTop = true } },
        onLogout = { logout(authViewModel, navController) },
    ) { padding ->
        androidx.compose.foundation.layout.Box(Modifier.padding(padding)) { content() }
    }
}

private fun logout(authViewModel: AuthViewModel, navController: NavHostController) {
    authViewModel.logout {
        navController.navigate(SellerRoutes.LOGIN) {
            popUpTo(0) { inclusive = true }
        }
    }
}
