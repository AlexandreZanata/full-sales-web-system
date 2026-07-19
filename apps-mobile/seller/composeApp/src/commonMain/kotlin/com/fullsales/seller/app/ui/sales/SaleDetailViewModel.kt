package com.fullsales.seller.app.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.catalog.StockBalancePrefetcher
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sales.SaleActionResult
import com.fullsales.seller.shared.sales.SaleActionSubmitter
import com.fullsales.seller.shared.sales.SaleDetailLoader
import com.fullsales.seller.shared.sales.SaleDetailModel
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class SaleDetailUiState(
    val loading: Boolean = true,
    val acting: Boolean = false,
    val detail: SaleDetailModel? = null,
    val stockByProductId: Map<String, Int> = emptyMap(),
    val errorCode: String? = null,
    val snackbarCode: String? = null,
    val navigateToNewSale: Boolean = false,
)

class SaleDetailViewModel(
    private val loader: SaleDetailLoader,
    private val actionSubmitter: SaleActionSubmitter,
    private val catalogRepository: CatalogRepository,
    private val networkMonitor: NetworkMonitor,
    private val stockPrefetcher: StockBalancePrefetcher,
) : ViewModel() {
    private val _state = MutableStateFlow(SaleDetailUiState())
    val state: StateFlow<SaleDetailUiState> = _state.asStateFlow()
    private var saleId: String? = null
    private var commerces: List<Commerce> = emptyList()
    private var products: List<Product> = emptyList()
    private var loadJob: Job? = null
    private var loadGeneration = 0

    init {
        viewModelScope.launch {
            val cached = stockPrefetcher.cachedMap()
            if (cached.isNotEmpty()) {
                _state.update { it.copy(stockByProductId = cached) }
            }
        }
        viewModelScope.launch {
            combine(
                catalogRepository.observeCommerces(),
                catalogRepository.observeProducts(),
            ) { commerceList, productList -> commerceList to productList }
                .collect { (commerceList, productList) ->
                    commerces = commerceList
                    products = productList
                    saleId?.let { fetchDetail(it, showLoading = _state.value.detail == null) }
                }
        }
    }

    fun load(id: String) {
        saleId = id
        fetchDetail(id, showLoading = true)
    }

    fun confirm() = runAction(
        block = { detail -> actionSubmitter.confirm(detail, networkMonitor.isOnline()) },
        onSuccess = { saleId?.let { fetchDetail(it, showLoading = false) } },
    )

    fun cancel() = runAction(
        block = { detail -> actionSubmitter.cancel(detail, networkMonitor.isOnline()) },
        onSuccess = { _state.update { it.copy(navigateToNewSale = true) } },
    )

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    fun consumeNavigateToNewSale() {
        _state.update { it.copy(navigateToNewSale = false) }
    }

    private fun fetchDetail(id: String, showLoading: Boolean) {
        loadJob?.cancel()
        val generation = ++loadGeneration
        loadJob = viewModelScope.launch {
            if (showLoading) _state.update { it.copy(loading = true, errorCode = null) }
            loader.load(id, commerces, products, networkMonitor.isOnline())
                .onSuccess { detail ->
                    if (generation == loadGeneration) {
                        _state.update { it.copy(loading = false, detail = detail) }
                        prefetchStockForDetail(detail)
                    }
                }
                .onFailure { error ->
                    if (generation == loadGeneration) {
                        _state.update { mapLoadError(error) }
                    }
                }
        }
    }

    private fun runAction(
        block: suspend (SaleDetailModel) -> SaleActionResult,
        onSuccess: () -> Unit,
    ) {
        val detail = _state.value.detail ?: return
        viewModelScope.launch {
            _state.update { it.copy(acting = true) }
            when (val result = block(detail)) {
                SaleActionResult.Success -> {
                    _state.update { it.copy(acting = false) }
                    onSuccess()
                }
                is SaleActionResult.Failure -> {
                    _state.update {
                        it.copy(acting = false, snackbarCode = result.code)
                    }
                }
            }
        }
    }

    private fun prefetchStockForDetail(detail: SaleDetailModel) {
        detail.items.map { it.productId }.distinct()
            .filter { it.isNotBlank() && it !in _state.value.stockByProductId }
            .forEach { productId ->
                viewModelScope.launch {
                    val available = if (networkMonitor.isOnline()) {
                        stockPrefetcher.fetchAndCache(productId)
                    } else {
                        stockPrefetcher.cachedMap()[productId]
                    } ?: return@launch
                    _state.update {
                        it.copy(stockByProductId = it.stockByProductId + (productId to available))
                    }
                }
            }
    }

    private fun mapLoadError(error: Throwable): SaleDetailUiState {
        val code = when (error) {
            is ApiException -> error.detail.code
            is IllegalStateException -> "OFFLINE_UNAVAILABLE"
            else -> "LOAD_FAILED"
        }
        return SaleDetailUiState(loading = false, errorCode = code)
    }
}
