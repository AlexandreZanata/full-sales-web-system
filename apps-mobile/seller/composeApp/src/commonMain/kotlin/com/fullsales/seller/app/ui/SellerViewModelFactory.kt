package com.fullsales.seller.app.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.ui.auth.AuthViewModel
import com.fullsales.seller.app.ui.commerces.CommerceDetailViewModel
import com.fullsales.seller.app.ui.commerces.CommerceViewModel
import com.fullsales.seller.app.ui.products.ProductDetailViewModel
import com.fullsales.seller.app.ui.products.ProductViewModel
import com.fullsales.seller.app.ui.sales.CreateSaleViewModel
import com.fullsales.seller.app.ui.sales.SaleDetailViewModel
import com.fullsales.seller.app.ui.sales.SalesListViewModel
import com.fullsales.seller.app.ui.settings.SettingsViewModel
import com.fullsales.seller.app.ui.sync.SyncStatusViewModel
import com.fullsales.seller.shared.sales.CreateSaleSubmitter
import com.fullsales.seller.shared.sales.SaleActionSubmitter
import com.fullsales.seller.shared.sales.SaleDetailLoader

class SellerViewModelFactory(
    private val container: SellerAppContainer,
) : ViewModelProvider.Factory {
    @Suppress("UNCHECKED_CAST")
    override fun <T : ViewModel> create(modelClass: Class<T>): T = when {
        modelClass.isAssignableFrom(AuthViewModel::class.java) ->
            AuthViewModel(container.apiClient, container.tokenStore) as T
        modelClass.isAssignableFrom(SettingsViewModel::class.java) ->
            SettingsViewModel(container.apiClient) as T
        modelClass.isAssignableFrom(SalesListViewModel::class.java) ->
            SalesListViewModel(
                container.apiClient,
                container.saleRepository,
                container.syncCoordinator,
                container.networkMonitor,
            ) as T
        modelClass.isAssignableFrom(CommerceViewModel::class.java) ->
            CommerceViewModel(
                container.catalogRepository,
                container.syncCoordinator,
                container.networkMonitor,
            ) as T
        modelClass.isAssignableFrom(CommerceDetailViewModel::class.java) ->
            CommerceDetailViewModel(container.apiClient) as T
        modelClass.isAssignableFrom(ProductViewModel::class.java) ->
            ProductViewModel(container.catalogRepository, container.syncCoordinator) as T
        modelClass.isAssignableFrom(ProductDetailViewModel::class.java) ->
            ProductDetailViewModel(container.apiClient, container.mediaUrlResolver) as T
        modelClass.isAssignableFrom(CreateSaleViewModel::class.java) ->
            CreateSaleViewModel(
                container.apiClient,
                container.catalogRepository,
                CreateSaleSubmitter(container.apiClient, container.offlineSaleWriter),
                container.networkMonitor,
            ) as T
        modelClass.isAssignableFrom(SaleDetailViewModel::class.java) ->
            SaleDetailViewModel(
                SaleDetailLoader(container.apiClient, container.saleRepository),
                SaleActionSubmitter(
                    container.apiClient,
                    container.saleRepository,
                    container.outboxRepository,
                    container.syncCoordinator,
                ),
                container.catalogRepository,
                container.networkMonitor,
            ) as T
        modelClass.isAssignableFrom(SyncStatusViewModel::class.java) ->
            SyncStatusViewModel(
                container,
                container.saleRepository,
                container.outboxRepository,
                container.networkMonitor,
            ) as T
        modelClass.isAssignableFrom(LocaleViewModel::class.java) ->
            LocaleViewModel() as T
        modelClass.isAssignableFrom(AccessibilityViewModel::class.java) ->
            AccessibilityViewModel() as T
        else -> error("Unknown ViewModel: ${modelClass.name}")
    }
}
