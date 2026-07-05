package com.fullsales.seller.android.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.fullsales.seller.android.AppContainer
import com.fullsales.seller.android.ui.auth.AuthViewModel
import com.fullsales.seller.android.ui.auth.LoginScreen
import com.fullsales.seller.android.ui.commerces.CommerceViewModel
import com.fullsales.seller.android.ui.sales.SalesListScreen
import com.fullsales.seller.android.ui.sales.SalesViewModel
import com.fullsales.seller.android.ui.settings.SettingsViewModel
import com.fullsales.seller.android.ui.sync.SyncStatusViewModel

@Composable
fun SellerNavHost(container: AppContainer) {
    val factory = SellerViewModelFactory(container)
    val authViewModel: AuthViewModel = viewModel(factory = factory)
    val settingsViewModel: SettingsViewModel = viewModel(factory = factory)
    val salesViewModel: SalesViewModel = viewModel(factory = factory)
    val commerceViewModel: CommerceViewModel = viewModel(factory = factory)
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
                    container.requestSync()
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
            NewSaleWithCommercePicker(navController, commerceViewModel)
        }
        detailRoute(SellerRoutes.SALE_DETAIL, "saleId", navController, settings, syncBadge, authViewModel) { id ->
            PlaceholderScreen("Sale $id", "Sale detail — Phase 61")
        }
        commerceRoutes(navController, factory, commerceViewModel, settings, syncBadge, authViewModel)
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
