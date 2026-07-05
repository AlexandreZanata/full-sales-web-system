package com.fullsales.seller.android.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.repository.SaleRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class SalesViewModel(
    private val saleRepository: SaleRepository,
) : ViewModel() {
    private val _sales = MutableStateFlow<List<LocalSale>>(emptyList())
    val sales: StateFlow<List<LocalSale>> = _sales.asStateFlow()

    init {
        viewModelScope.launch {
            saleRepository.observeSales().collect { _sales.value = it }
        }
    }
}
