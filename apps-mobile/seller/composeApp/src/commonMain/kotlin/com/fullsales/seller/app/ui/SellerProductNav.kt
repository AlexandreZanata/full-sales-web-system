package com.fullsales.seller.app.ui

import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.auth.AuthViewModel
import com.fullsales.seller.app.ui.products.ProductDetailScreen
import com.fullsales.seller.app.ui.products.ProductDetailViewModel
import com.fullsales.seller.app.ui.products.ProductListScreen
import com.fullsales.seller.app.ui.products.ProductViewModel
import com.fullsales.seller.app.ui.sync.SyncBadge

internal const val SELECTED_PRODUCT_ID = "selectedProductId"

internal fun NavGraphBuilder.productRoutes(
    navController: NavHostController,
    factory: SellerViewModelFactory,
    productViewModel: ProductViewModel,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
) {
    composable(SellerRoutes.PRODUCTS) {
        DetailShell(navController, syncBadge, authViewModel, localeViewModel, accessibilityViewModel) {
            ProductListScreen(
                viewModel = productViewModel,
                mediaUrlResolver = factory.mediaUrlResolver,
                onProductClick = { id -> navController.navigate(SellerRoutes.productDetail(id)) },
            )
        }
    }
    detailRoute(
        SellerRoutes.PRODUCT_DETAIL,
        "productId",
        navController,
        syncBadge,
        authViewModel,
        localeViewModel,
        accessibilityViewModel,
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
