package com.fullsales.seller.android.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import com.fullsales.seller.android.ui.auth.AuthViewModel
import com.fullsales.seller.android.ui.commerces.CommerceDetailScreen
import com.fullsales.seller.android.ui.commerces.CommerceDetailViewModel
import com.fullsales.seller.android.ui.commerces.CommerceListScreen
import com.fullsales.seller.android.ui.commerces.CommerceViewModel
import com.fullsales.seller.android.ui.sales.CreateSaleScreen
import com.fullsales.seller.android.ui.settings.SettingsUiState
import com.fullsales.seller.android.ui.sync.SyncBadge
import com.fullsales.seller.shared.model.displayName

internal const val SELECTED_COMMERCE_ID = "selectedCommerceId"

internal fun NavGraphBuilder.commerceRoutes(
    navController: NavHostController,
    factory: SellerViewModelFactory,
    commerceViewModel: CommerceViewModel,
    settings: SettingsUiState,
    syncBadge: SyncBadge,
    authViewModel: AuthViewModel,
) {
    composable(SellerRoutes.COMMERCES) {
        DetailShell(navController, settings, syncBadge, authViewModel) {
            CommerceListScreen(
                viewModel = commerceViewModel,
                onCommerceClick = { id -> navController.navigate(SellerRoutes.commerceDetail(id)) },
            )
        }
    }
    composable(SellerRoutes.COMMERCE_PICK) {
        DetailShell(navController, settings, syncBadge, authViewModel) {
            CommerceListScreen(
                viewModel = commerceViewModel,
                onCommerceClick = {},
                onPick = { id ->
                    navController.previousBackStackEntry
                        ?.savedStateHandle
                        ?.set(SELECTED_COMMERCE_ID, id)
                    navController.popBackStack()
                },
                title = "Select commerce",
            )
        }
    }
    detailRoute(SellerRoutes.COMMERCE_DETAIL, "commerceId", navController, settings, syncBadge, authViewModel) { id ->
        val detailViewModel: CommerceDetailViewModel = viewModel(factory = factory)
        CommerceDetailScreen(commerceId = id, viewModel = detailViewModel)
    }
}

@Composable
internal fun NewSaleWithCommercePicker(
    navController: NavHostController,
    commerceViewModel: CommerceViewModel,
) {
    val newSaleEntry = navController.currentBackStackEntry
    val pickerResult = newSaleEntry?.savedStateHandle?.getStateFlow<String?>(SELECTED_COMMERCE_ID, null)
    val selectedId by pickerResult?.collectAsState() ?: remember { mutableStateOf<String?>(null) }
    val commerceState by commerceViewModel.state.collectAsState()
    val selectedLabel = commerceState.items.firstOrNull { it.id == selectedId }?.displayName()
    CreateSaleScreen(
        selectedCommerceLabel = selectedLabel,
        onOpenCommercePicker = { navController.navigate(SellerRoutes.COMMERCE_PICK) },
        onBrowseCommerces = { navController.navigate(SellerRoutes.COMMERCES) },
    )
}
