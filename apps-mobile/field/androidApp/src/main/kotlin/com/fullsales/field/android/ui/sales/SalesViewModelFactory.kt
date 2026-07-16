package com.fullsales.field.android.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import com.fullsales.field.android.AppContainer

class SalesViewModelFactory(
    private val container: AppContainer,
) : ViewModelProvider.Factory {
    @Suppress("UNCHECKED_CAST")
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(SalesViewModel::class.java)) {
            return SalesViewModel(
                container.catalogRepository,
                container.saleRepository,
                container.offlineSaleWriter,
                container.networkMonitor,
                container.apiClient,
            ) as T
        }
        throw IllegalArgumentException("Unknown ViewModel ${modelClass.name}")
    }
}
