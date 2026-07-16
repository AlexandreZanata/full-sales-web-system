package com.fullsales.seller.app.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewmodel.CreationExtras
import kotlin.reflect.KClass
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.platform.CommerceRegistrationDraftStore
import com.fullsales.seller.app.platform.CreateSaleDraftStore
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.ui.auth.AuthViewModel
import com.fullsales.seller.app.ui.commerces.CommerceDetailViewModel
import com.fullsales.seller.app.ui.commerces.CommerceViewModel
import com.fullsales.seller.app.ui.commerces.registrations.CnpjLookupViewModel
import com.fullsales.seller.app.ui.commerces.registrations.CommerceRegistrationViewModel
import com.fullsales.seller.app.ui.commerces.registrations.MyRegistrationsViewModel
import com.fullsales.seller.app.ui.products.ProductDetailViewModel
import com.fullsales.seller.app.ui.products.ProductViewModel
import com.fullsales.seller.app.ui.sales.CreateSaleViewModel
import com.fullsales.seller.app.ui.sales.SaleDetailViewModel
import com.fullsales.seller.app.ui.sales.SalesListViewModel
import com.fullsales.seller.app.ui.settings.SettingsViewModel
import com.fullsales.seller.app.ui.offline.OfflineHubViewModel
import com.fullsales.seller.app.ui.sync.SyncStatusViewModel
import com.fullsales.seller.shared.catalog.CommerceDetailLoader
import com.fullsales.seller.shared.catalog.ProductDetailLoader
import com.fullsales.seller.shared.catalog.StockBalancePrefetcher
import com.fullsales.seller.shared.registrations.CreateRegistrationSubmitter
import com.fullsales.seller.shared.sales.CreateSaleSubmitter
import com.fullsales.seller.shared.sales.SaleActionSubmitter
import com.fullsales.seller.shared.sales.SaleDetailLoader

class SellerViewModelFactory(
    private val container: SellerAppContainer,
) : ViewModelProvider.Factory {
    val mediaUrlResolver: MediaUrlResolver get() = container.mediaUrlResolver
    private val stockPrefetcher
        get() = StockBalancePrefetcher(container.apiClient, container.stockSnapshots)

    @Suppress("UNCHECKED_CAST")
    override fun <T : ViewModel> create(modelClass: KClass<T>, extras: CreationExtras): T = when (modelClass) {
        AuthViewModel::class ->
            AuthViewModel(container.apiClient, container.tokenStore) as T
        SettingsViewModel::class ->
            SettingsViewModel(
                container.apiClient,
                container.siteSettingsRepository,
                container.networkMonitor,
            ) as T
        SalesListViewModel::class ->
            SalesListViewModel(
                container.saleRepository,
                container.syncCoordinator,
                container.networkMonitor,
            ) as T
        CommerceViewModel::class ->
            CommerceViewModel(
                container.catalogRepository,
                container.syncCoordinator,
                container.networkMonitor,
            ) as T
        CommerceDetailViewModel::class ->
            CommerceDetailViewModel(
                CommerceDetailLoader(
                    container.catalogRepository,
                    container.commerceAddressCache,
                    container.apiClient,
                ),
                container.networkMonitor,
            ) as T
        CnpjLookupViewModel::class ->
            CnpjLookupViewModel(container.apiClient, container.networkMonitor) as T
        CommerceRegistrationViewModel::class ->
            CommerceRegistrationViewModel(
                CreateRegistrationSubmitter(
                    container.apiClient,
                    container.offlineRegistrationWriter,
                    container.registrationRepository,
                ),
                container.networkMonitor,
                container.syncCoordinator,
                CommerceRegistrationDraftStore(),
            ) as T
        MyRegistrationsViewModel::class ->
            MyRegistrationsViewModel(
                container.registrationRepository,
                container.syncCoordinator,
                container.networkMonitor,
            ) as T
        ProductViewModel::class ->
            ProductViewModel(
                container.catalogRepository,
                container.syncCoordinator,
                container.networkMonitor,
                stockPrefetcher,
            ) as T
        ProductDetailViewModel::class ->
            ProductDetailViewModel(
                ProductDetailLoader(
                    container.catalogRepository,
                    container.stockSnapshots,
                    container.apiClient,
                ),
                container.mediaUrlResolver,
                container.networkMonitor,
            ) as T
        CreateSaleViewModel::class ->
            CreateSaleViewModel(
                container.apiClient,
                container.catalogRepository,
                CreateSaleSubmitter(
                    container.apiClient,
                    container.offlineSaleWriter,
                    container.saleRepository,
                ),
                container.networkMonitor,
                CreateSaleDraftStore(),
                stockPrefetcher,
            ) as T
        SaleDetailViewModel::class ->
            SaleDetailViewModel(
                SaleDetailLoader(
                    container.apiClient,
                    container.saleRepository,
                    container.outboxRepository,
                ),
                SaleActionSubmitter(
                    container.apiClient,
                    container.saleRepository,
                    container.outboxRepository,
                    container.syncCoordinator,
                ),
                container.catalogRepository,
                container.networkMonitor,
                stockPrefetcher,
            ) as T
        SyncStatusViewModel::class ->
            SyncStatusViewModel(
                container,
                container.saleRepository,
                container.outboxRepository,
                container.networkMonitor,
            ) as T
        OfflineHubViewModel::class ->
            OfflineHubViewModel(
                container,
                container.catalogRepository,
                container.saleRepository,
                container.registrationRepository,
                container.outboxRepository,
                container.networkMonitor,
            ) as T
        LocaleViewModel::class ->
            LocaleViewModel() as T
        AccessibilityViewModel::class ->
            AccessibilityViewModel() as T
        else -> error("Unknown ViewModel: ${modelClass.simpleName}")
    }
}
