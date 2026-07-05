package com.fullsales.seller.android.ui.sales

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
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
    context: Context,
) : ViewModel() {
    private val connectivity = context.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
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
            loader.load(id, commerces, products, isOnline())
                .onSuccess { detail -> _state.update { it.copy(loading = false, detail = detail) } }
                .onFailure { error -> _state.update { mapLoadError(error) } }
        }
    }

    fun confirm() = runAction { detail -> actionSubmitter.confirm(detail, isOnline()) }

    fun cancel() = runAction { detail -> actionSubmitter.cancel(detail, isOnline()) }

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

    private fun isOnline(): Boolean {
        val network = connectivity.activeNetwork ?: return false
        val caps = connectivity.getNetworkCapabilities(network) ?: return false
        return caps.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }
}
