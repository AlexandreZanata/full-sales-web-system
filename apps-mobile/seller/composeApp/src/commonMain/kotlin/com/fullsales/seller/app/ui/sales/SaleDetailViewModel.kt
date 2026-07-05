package com.fullsales.seller.app.ui.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.sales.SaleActionResult
import com.fullsales.seller.shared.sales.SaleActionSubmitter
import com.fullsales.seller.shared.sales.SaleDetailLoader
import com.fullsales.seller.shared.sales.SaleDetailModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class SaleDetailUiState(
    val loading: Boolean = true,
    val acting: Boolean = false,
    val detail: SaleDetailModel? = null,
    val errorCode: String? = null,
    val snackbarCode: String? = null,
)

class SaleDetailViewModel(
    private val loader: SaleDetailLoader,
    private val actionSubmitter: SaleActionSubmitter,
    private val catalogRepository: CatalogRepository,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(SaleDetailUiState())
    val state: StateFlow<SaleDetailUiState> = _state.asStateFlow()
    private var commerces: List<Commerce> = emptyList()
    private var products: List<Product> = emptyList()
    private var saleId: String? = null

    init {
        viewModelScope.launch {
            catalogRepository.observeCommerces().collect { commerces = it }
        }
        viewModelScope.launch {
            catalogRepository.observeProducts().collect { products = it }
        }
    }

    fun load(id: String) {
        saleId = id
        viewModelScope.launch {
            _state.update { it.copy(loading = true, errorCode = null) }
            loader.load(id, commerces, products, networkMonitor.isOnline())
                .onSuccess { detail -> _state.update { it.copy(loading = false, detail = detail) } }
                .onFailure { error -> _state.update { mapLoadError(error) } }
        }
    }

    fun confirm() = runAction { detail -> actionSubmitter.confirm(detail, networkMonitor.isOnline()) }

    fun cancel() = runAction { detail -> actionSubmitter.cancel(detail, networkMonitor.isOnline()) }

    fun clearSnackbar() {
        _state.update { it.copy(snackbarCode = null) }
    }

    private fun runAction(block: suspend (SaleDetailModel) -> SaleActionResult) {
        val detail = _state.value.detail ?: return
        viewModelScope.launch {
            _state.update { it.copy(acting = true) }
            when (val result = block(detail)) {
                SaleActionResult.Success -> {
                    _state.update { it.copy(acting = false) }
                    saleId?.let { load(it) }
                }
                is SaleActionResult.Failure -> {
                    _state.update {
                        it.copy(acting = false, snackbarCode = result.code)
                    }
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
