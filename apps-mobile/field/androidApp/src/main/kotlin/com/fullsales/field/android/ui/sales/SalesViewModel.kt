package com.fullsales.field.android.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.field.shared.model.Commerce
import com.fullsales.field.shared.model.CreateSaleItem
import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.Product
import com.fullsales.field.shared.model.Sale
import com.fullsales.field.shared.repository.CatalogRepository
import com.fullsales.field.shared.repository.SaleRepository
import com.fullsales.field.shared.sync.OfflineSaleWriter
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class NewSaleUiState(
    val commerces: List<Commerce> = emptyList(),
    val products: List<Product> = emptyList(),
    val loading: Boolean = true,
    val saving: Boolean = false,
    val error: String? = null,
)

class SalesViewModel(
    private val catalog: CatalogRepository,
    private val saleRepo: SaleRepository,
    private val offlineWriter: OfflineSaleWriter,
) : ViewModel() {
    private val _sales = MutableStateFlow<List<Sale>>(emptyList())
    val sales: StateFlow<List<Sale>> = _sales.asStateFlow()
    private val _newSale = MutableStateFlow(NewSaleUiState())
    val newSale: StateFlow<NewSaleUiState> = _newSale.asStateFlow()

    init {
        refreshSales()
    }

    fun refreshSales() {
        viewModelScope.launch {
            _sales.value = saleRepo.listSales()
        }
    }

    fun loadCatalog() {
        viewModelScope.launch {
            _newSale.value = _newSale.value.copy(loading = true, error = null)
            runCatching {
                _newSale.value = _newSale.value.copy(
                    commerces = catalog.listActiveCommerces(),
                    products = catalog.listActiveProducts(),
                    loading = false,
                )
            }.onFailure {
                _newSale.value = _newSale.value.copy(loading = false, error = it.message)
            }
        }
    }

    fun createOfflineSale(
        commerceId: String,
        paymentMethod: String,
        lines: List<Pair<String, Int>>,
        onDone: () -> Unit,
    ) {
        viewModelScope.launch {
            _newSale.value = _newSale.value.copy(saving = true, error = null)
            val products = _newSale.value.products
            val total = lines.sumOf { (productId, qty) ->
                (products.find { it.id == productId }?.priceAmount ?: 0.0) * qty
            }
            val localId = UUID.randomUUID().toString()
            val request = CreateSaleRequest(
                commerceId = commerceId,
                paymentMethod = paymentMethod,
                items = lines.map { CreateSaleItem(it.first, it.second) },
            )
            runCatching {
                offlineWriter.createSale(localId, request, total)
                refreshSales()
                onDone()
            }.onFailure {
                _newSale.value = _newSale.value.copy(error = it.message)
            }
            _newSale.value = _newSale.value.copy(saving = false)
        }
    }
}
