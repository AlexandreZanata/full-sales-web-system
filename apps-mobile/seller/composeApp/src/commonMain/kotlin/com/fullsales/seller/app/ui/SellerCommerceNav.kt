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
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.auth.AuthViewModel
import com.fullsales.seller.app.ui.commerces.CommerceDetailScreen
import com.fullsales.seller.app.ui.commerces.CommerceDetailViewModel
import com.fullsales.seller.app.ui.commerces.CommerceListScreen
import com.fullsales.seller.app.ui.commerces.CommerceViewModel
import com.fullsales.seller.app.ui.commerces.registrations.CnpjLookupScreen
import com.fullsales.seller.app.ui.commerces.registrations.CnpjLookupViewModel
import com.fullsales.seller.app.ui.commerces.registrations.CommerceRegistrationFormScreen
import com.fullsales.seller.app.ui.commerces.registrations.CommerceRegistrationViewModel
import com.fullsales.seller.app.ui.commerces.registrations.MyRegistrationsScreen
import com.fullsales.seller.app.ui.commerces.registrations.MyRegistrationsViewModel
import com.fullsales.seller.app.ui.commerces.registrations.RegistrationModeScreen
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
    accessibilityViewModel: AccessibilityViewModel,
) {
    composable(SellerRoutes.COMMERCE_REGISTRATION_MODE) {
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel, accessibilityViewModel) {
            val registrationViewModel: CommerceRegistrationViewModel = viewModel(factory = factory)
            RegistrationModeScreen(
                onCnpjLookup = { navController.navigate(SellerRoutes.COMMERCE_REGISTRATION_CNPJ) },
                onManual = {
                    registrationViewModel.startManual()
                    navController.navigate(SellerRoutes.COMMERCE_REGISTRATION_FORM)
                },
                onMyRegistrations = { navController.navigate(SellerRoutes.COMMERCE_REGISTRATIONS) },
            )
        }
    }
    composable(SellerRoutes.COMMERCE_REGISTRATION_CNPJ) {
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel, accessibilityViewModel) {
            val lookupViewModel: CnpjLookupViewModel = viewModel(factory = factory)
            val registrationViewModel: CommerceRegistrationViewModel = viewModel(factory = factory)
            CnpjLookupScreen(
                viewModel = lookupViewModel,
                onContinue = { result ->
                    registrationViewModel.applyLookupResult(result)
                    navController.navigate(SellerRoutes.COMMERCE_REGISTRATION_FORM)
                },
            )
        }
    }
    composable(SellerRoutes.COMMERCE_REGISTRATION_FORM) {
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel, accessibilityViewModel) {
            val registrationViewModel: CommerceRegistrationViewModel = viewModel(factory = factory)
            CommerceRegistrationFormScreen(
                viewModel = registrationViewModel,
                onSubmitted = {
                    navController.navigate(SellerRoutes.COMMERCE_REGISTRATIONS) {
                        popUpTo(SellerRoutes.COMMERCE_REGISTRATION_MODE) { inclusive = false }
                    }
                },
                onBack = { navController.popBackStack() },
            )
        }
    }
    composable(SellerRoutes.COMMERCE_REGISTRATIONS) {
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel, accessibilityViewModel) {
            val registrationsViewModel: MyRegistrationsViewModel = viewModel(factory = factory)
            MyRegistrationsScreen(viewModel = registrationsViewModel)
        }
    }
    composable(SellerRoutes.COMMERCE_PICK) {
        LaunchedEffect(Unit) { commerceViewModel.refresh() }
        DetailShell(navController, settings, syncBadge, authViewModel, localeViewModel, accessibilityViewModel) {
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
        accessibilityViewModel,
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
        mediaUrlResolver = factory.mediaUrlResolver,
        onBack = { navController.popBackStack() },
        onCreated = { id -> navController.navigate(SellerRoutes.saleDetail(id)) },
        onOpenCommercePicker = { navController.navigate(SellerRoutes.COMMERCE_PICK) },
    )
}
