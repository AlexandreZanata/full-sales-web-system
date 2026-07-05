package com.fullsales.seller.android.ui

import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import com.fullsales.seller.android.i18n.LocaleViewModel
import com.fullsales.seller.android.ui.auth.AuthViewModel
import com.fullsales.seller.android.ui.products.ProductDetailScreen
import com.fullsales.seller.android.ui.products.ProductDetailViewModel
import com.fullsales.seller.android.ui.products.ProductListScreen
import com.fullsales.seller.android.ui.products.ProductViewModel
import com.fullsales.seller.android.ui.settings.SettingsUiState
import com.fullsales.seller.android.ui.sync.SyncBadge

internal const val SELECTED_PRODUCT_ID = "selectedProductId"

internal fun NavGraphBuilder.productRoutes(
    navController: NavHostController,
    factory: SellerViewModelFactory,
    productViewModel: ProductViewModel,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
) {
    composable(SellerRoutes.PRODUCTS) {
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel) {
            ProductListScreen(
                viewModel = productViewModel,
                onProductClick = { id -> navController.navigate(SellerRoutes.productDetail(id)) },
            )
        }
    }
    detailRoute(
        SellerRoutes.PRODUCT_DETAIL,
        "productId",
        navController,
        settings,
        syncBadge,
        authViewModel,
        localeViewModel,
    ) { id ->
        val detailViewModel: ProductDetailViewModel = viewModel(factory = factory)
        ProductDetailScreen(
            productId = id,
            viewModel = detailViewModel,
            onAddToSale = { productId -> navigateToNewSaleWithProduct(navController, productId) },
        )
    }
}

internal fun navigateToNewSaleWithProduct(navController: NavHostController, productId: String) {
    navController.navigate(SellerRoutes.SALES_NEW) {
        launchSingleTop = true
    }
    navController.getBackStackEntry(SellerRoutes.SALES_NEW)
        .savedStateHandle[SELECTED_PRODUCT_ID] = productId
}
