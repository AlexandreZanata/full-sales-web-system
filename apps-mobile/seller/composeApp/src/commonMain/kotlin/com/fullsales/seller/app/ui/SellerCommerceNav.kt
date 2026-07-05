package com.fullsales.seller.app.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.auth.AuthViewModel
import com.fullsales.seller.app.ui.commerces.CommerceDetailScreen
import com.fullsales.seller.app.ui.commerces.CommerceDetailViewModel
import com.fullsales.seller.app.ui.commerces.CommerceListScreen
import com.fullsales.seller.app.ui.commerces.CommerceViewModel
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.sales.CreateSaleScreen
import com.fullsales.seller.app.ui.sales.CreateSaleViewModel
import com.fullsales.seller.app.ui.settings.SettingsUiState
import com.fullsales.seller.app.ui.sync.SyncBadge

internal const val SELECTED_COMMERCE_ID = "selectedCommerceId"

internal fun NavGraphBuilder.commerceRoutes(
    navController: NavHostController,
    factory: SellerViewModelFactory,
    commerceViewModel: CommerceViewModel,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
) {
    composable(SellerRoutes.COMMERCES) {
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel) {
            CommerceListScreen(
                viewModel = commerceViewModel,
                onCommerceClick = { id -> navController.navigate(SellerRoutes.commerceDetail(id)) },
            )
        }
    }
    composable(SellerRoutes.COMMERCE_PICK) {
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel) {
            val s = LocalSellerStrings.current
            CommerceListScreen(
                viewModel = commerceViewModel,
                onCommerceClick = {},
                onPick = { id ->
                    navController.previousBackStackEntry
                        ?.savedStateHandle
                        ?.set(SELECTED_COMMERCE_ID, id)
                    navController.popBackStack()
                },
                title = s.commerces.selectTitle,
            )
        }
    }
    detailRoute(
        SellerRoutes.COMMERCE_DETAIL,
        "commerceId",
        navController,
        settings,
        syncBadge,
        authViewModel,
        localeViewModel,
    ) { id ->
        val detailViewModel: CommerceDetailViewModel = viewModel(factory = factory)
        CommerceDetailScreen(commerceId = id, viewModel = detailViewModel)
    }
}

@Composable
internal fun NewSaleWithCommercePicker(
    navController: NavHostController,
    factory: SellerViewModelFactory,
) {
    val createSaleViewModel: CreateSaleViewModel = viewModel(factory = factory)
    val newSaleEntry = navController.currentBackStackEntry
    val commerceResult = newSaleEntry?.savedStateHandle?.getStateFlow<String?>(SELECTED_COMMERCE_ID, null)
    val productResult = newSaleEntry?.savedStateHandle?.getStateFlow<String?>(SELECTED_PRODUCT_ID, null)
    val selectedCommerceId by commerceResult?.collectAsState() ?: remember { mutableStateOf<String?>(null) }
    val selectedProductId by productResult?.collectAsState() ?: remember { mutableStateOf<String?>(null) }
    LaunchedEffect(selectedCommerceId) {
        selectedCommerceId?.let(createSaleViewModel::setCommerceId)
    }
    LaunchedEffect(selectedProductId) {
        selectedProductId?.let(createSaleViewModel::prefillProduct)
    }
    CreateSaleScreen(
        viewModel = createSaleViewModel,
        onBack = { navController.popBackStack() },
        onCreated = { id -> navController.navigate(SellerRoutes.saleDetail(id)) },
        onOpenCommercePicker = { navController.navigate(SellerRoutes.COMMERCE_PICK) },
    )
}
