package com.fullsales.seller.android.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import com.fullsales.seller.android.AppContainer
import com.fullsales.seller.android.ui.auth.AuthViewModel
import com.fullsales.seller.android.ui.commerces.CommerceDetailViewModel
import com.fullsales.seller.android.ui.commerces.CommerceViewModel
import com.fullsales.seller.android.ui.sales.SalesViewModel
import com.fullsales.seller.android.ui.settings.SettingsViewModel
import com.fullsales.seller.android.ui.sync.SyncStatusViewModel

class SellerViewModelFactory(
    private val container: AppContainer,
) : ViewModelProvider.Factory {
    @Suppress("UNCHECKED_CAST")
    override fun <T : ViewModel> create(modelClass: Class<T>): T = when {
        modelClass.isAssignableFrom(AuthViewModel::class.java) ->
            AuthViewModel(container.apiClient, container.tokenStore) as T
        modelClass.isAssignableFrom(SettingsViewModel::class.java) ->
            SettingsViewModel(container.apiClient) as T
        modelClass.isAssignableFrom(SalesViewModel::class.java) ->
            SalesViewModel(container.saleRepository) as T
        modelClass.isAssignableFrom(CommerceViewModel::class.java) ->
            CommerceViewModel(container.catalogRepository, container.syncCoordinator) as T
        modelClass.isAssignableFrom(CommerceDetailViewModel::class.java) ->
            CommerceDetailViewModel(container.apiClient) as T
        modelClass.isAssignableFrom(SyncStatusViewModel::class.java) ->
            SyncStatusViewModel(
                container,
                container.saleRepository,
                container.outboxRepository,
                container.appContext,
            ) as T
        else -> error("Unknown ViewModel: ${modelClass.name}")
    }
}
