package com.fullsales.seller.app.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.TopSellingProduct
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sales.CreateSaleFormErrors
import com.fullsales.seller.shared.sales.CreateSaleLineInput
import com.fullsales.seller.shared.sales.CreateSaleSubmitter
import com.fullsales.seller.shared.sales.CreateSaleSubmitResult
import com.fullsales.seller.shared.sales.buildCreateSaleRequest
import com.fullsales.seller.shared.sales.calculateCreateSaleTotalMinor
import com.fullsales.seller.shared.sales.validateCreateSaleForm
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class CreateSaleUiState(
    val commerces: List<Commerce> = emptyList(),
    val products: List<Product> = emptyList(),
    val topSellingProducts: List<TopSellingProduct> = emptyList(),
    val commerceId: String = "",
    val paymentMethod: String = "",
    val lines: List<CreateSaleLineInput> = listOf(CreateSaleLineInput()),
    val stockByProductId: Map<String, Int> = emptyMap(),
    val loading: Boolean = true,
    val submitting: Boolean = false,
    val errors: CreateSaleFormErrors = CreateSaleFormErrors(),
    val snackbarCode: String? = null,
) {
    val totalMinor: Long
        get() = calculateCreateSaleTotalMinor(products, lines)
}

class CreateSaleViewModel(
    private val apiClient: SellerApiClient,
    private val catalogRepository: CatalogRepository,
    private val submitter: CreateSaleSubmitter,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(CreateSaleUiState())
    val state: StateFlow<CreateSaleUiState> = _state.asStateFlow()

    init {
        viewModelScope.launch {
            combine(
                catalogRepository.observeCommerces(),
                catalogRepository.observeProducts(),
            ) { commerces, products ->
                commerces to products
            }.collect { (commerces, products) ->
                _state.update { it.copy(commerces = commerces, products = products, loading = false) }
            }
        }
        loadTopSellingProducts()
    }

    private fun loadTopSellingProducts() {
        viewModelScope.launch {
            if (!networkMonitor.isOnline()) return@launch
            runCatching { apiClient.listTopSellingProducts(limit = 5) }
                .onSuccess { response ->
                    _state.update { it.copy(topSellingProducts = response.data) }
                }
        }
    }

    fun setCommerceId(id: String) {
        _state.update { it.copy(commerceId = id, errors = it.errors.copy(commerceError = null)) }
    }

    fun setPaymentMethod(method: String) {
        _state.update { it.copy(paymentMethod = method, errors = it.errors.copy(paymentError = null)) }
    }

    fun updateLine(index: Int, line: CreateSaleLineInput) {
        _state.update { current ->
            val next = current.lines.toMutableList()
            if (index in next.indices) next[index] = line
            current.copy(lines = next)
        }
        if (line.productId.isNotBlank()) loadStock(line.productId)
    }

    fun addLine() {
        _state.update { it.copy(lines = it.lines + CreateSaleLineInput()) }
    }

    fun removeLine(index: Int) {
        _state.update { current ->
            if (current.lines.size <= 1) return@update current
            current.copy(lines = current.lines.filterIndexed { i, _ -> i != index })
        }
    }

    fun prefillProduct(productId: String) {
        _state.update { current ->
            val first = current.lines.firstOrNull() ?: CreateSaleLineInput()
            current.copy(lines = listOf(first.copy(productId = productId)) + current.lines.drop(1))
        }
        loadStock(productId)
    }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun submit(onSuccess: (String) -> Unit) {
        val snapshot = _state.value
        val errors = validateCreateSaleForm(
            snapshot.commerceId,
            snapshot.paymentMethod,
            snapshot.lines,
            snapshot.stockByProductId,
        )
        if (!errors.isValid) {
            _state.update { it.copy(errors = errors) }
            return
        }
        viewModelScope.launch {
            _state.update { it.copy(submitting = true, errors = CreateSaleFormErrors()) }
            val request = buildCreateSaleRequest(
                snapshot.commerceId,
                snapshot.paymentMethod,
                snapshot.lines,
            )
            when (
                val result = submitter.submit(
                    request,
                    snapshot.totalMinor.toDouble(),
                    online = networkMonitor.isOnline(),
                )
            ) {
                is CreateSaleSubmitResult.Success -> {
                    _state.update { it.copy(submitting = false) }
                    onSuccess(result.navigationId)
                }
                is CreateSaleSubmitResult.Failure -> {
                    _state.update {
                        it.copy(submitting = false, snackbarCode = result.code)
                    }
                }
            }
        }
    }

    private fun loadStock(productId: String) {
        viewModelScope.launch {
            runCatching { apiClient.getStockBalance(productId).available }
                .onSuccess { available ->
                    _state.update { it.copy(stockByProductId = it.stockByProductId + (productId to available)) }
                }
        }
    }
}
