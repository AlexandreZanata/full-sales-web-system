package com.fullsales.seller.app.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.auth.AuthViewModel
import com.fullsales.seller.app.ui.auth.LoginScreen
import com.fullsales.seller.app.ui.commerces.CommerceViewModel
import com.fullsales.seller.app.ui.i18n.SellerStringsProvider
import com.fullsales.seller.app.ui.products.ProductViewModel
import com.fullsales.seller.app.ui.sales.SaleDetailScreen
import com.fullsales.seller.app.ui.sales.SaleDetailViewModel
import com.fullsales.seller.app.ui.sales.SalesListScreen
import com.fullsales.seller.app.ui.sales.SalesListViewModel
import com.fullsales.seller.app.ui.settings.SettingsViewModel
import com.fullsales.seller.app.ui.sync.SyncStatusViewModel
import com.fullsales.seller.app.ui.theme.SellerTheme

@Composable
fun SellerNavHost(container: SellerAppContainer) {
    val factory = SellerViewModelFactory(container)
    val localeViewModel: LocaleViewModel = viewModel(factory = factory)
    val accessibilityViewModel: AccessibilityViewModel = viewModel(factory = factory)
    val textSizePreset by accessibilityViewModel.preset.collectAsState()
    SellerStringsProvider(localeViewModel) {
        SellerTheme(textSizePreset = textSizePreset) {
            SellerNavHostContent(container, factory, localeViewModel, accessibilityViewModel)
        }
    }
}

@Composable
private fun SellerNavHostContent(
    container: SellerAppContainer,
    factory: SellerViewModelFactory,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
) {
    val authViewModel: AuthViewModel = viewModel(factory = factory)
    val settingsViewModel: SettingsViewModel = viewModel(factory = factory)
    val salesListViewModel: SalesListViewModel = viewModel(factory = factory)
    val commerceViewModel: CommerceViewModel = viewModel(factory = factory)
    val productViewModel: ProductViewModel = viewModel(factory = factory)
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
                localeViewModel = localeViewModel,
                accessibilityViewModel = accessibilityViewModel,
                onLoggedIn = {
                    settingsViewModel.loadIfStale(force = true)
                    container.requestSync()
                    navController.navigate(SellerRoutes.SALES) {
                        popUpTo(SellerRoutes.LOGIN) { inclusive = true }
                    }
                },
            )
        }
        shellRoute(
            SellerRoutes.SALES,
            navController,
            settings,
            syncBadge,
            authViewModel,
            settingsViewModel,
            localeViewModel,
            accessibilityViewModel,
        ) {
            SalesListScreen(
                viewModel = salesListViewModel,
                onSaleClick = { id -> navController.navigate(SellerRoutes.saleDetail(id)) },
                onNewSale = { navController.navigate(SellerRoutes.SALES_NEW) },
            )
        }
        shellRoute(
            SellerRoutes.SALES_NEW,
            navController,
            settings,
            syncBadge,
            authViewModel,
            settingsViewModel,
            localeViewModel,
            accessibilityViewModel,
        ) {
            NewSaleWithCommercePicker(navController, factory)
        }
        detailRoute(
            SellerRoutes.SALE_DETAIL,
            "saleId",
            navController,
            settings,
            syncBadge,
            authViewModel,
            localeViewModel,
            accessibilityViewModel,
        ) { id ->
            val viewModel: SaleDetailViewModel = viewModel(factory = factory)
            SaleDetailScreen(saleId = id, viewModel = viewModel)
        }
        commerceRoutes(
            navController,
            factory,
            commerceViewModel,
            settings,
            syncBadge,
            authViewModel,
            localeViewModel,
            accessibilityViewModel,
        )
        productRoutes(
            navController,
            factory,
            productViewModel,
            settings,
            syncBadge,
            authViewModel,
            localeViewModel,
            accessibilityViewModel,
        )
    }
}
