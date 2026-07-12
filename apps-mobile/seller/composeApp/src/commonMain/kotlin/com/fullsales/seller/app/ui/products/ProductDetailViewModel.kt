package com.fullsales.seller.app.ui.products

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.platform.NetworkMonitor
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.catalog.ProductDetailLoader
import com.fullsales.seller.shared.model.ProductDetail
import com.fullsales.seller.shared.model.formatProductPrice
import com.fullsales.seller.shared.model.isStockUnavailable
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class ProductDetailUiState(
    val loading: Boolean = true,
    val errorCode: String? = null,
    val product: ProductDetail? = null,
    val priceLabel: String? = null,
    val stockAvailable: Int? = null,
    val stockUnavailable: Boolean = false,
    val imageUrl: String? = null,
)

class ProductDetailViewModel(
    private val loader: ProductDetailLoader,
    private val mediaUrlResolver: MediaUrlResolver,
    private val networkMonitor: NetworkMonitor,
) : ViewModel() {
    private val _state = MutableStateFlow(ProductDetailUiState())
    val state: StateFlow<ProductDetailUiState> = _state.asStateFlow()

    fun load(productId: String) {
        viewModelScope.launch {
            _state.value = ProductDetailUiState(loading = true)
            runCatching {
                val loaded = loader.load(productId, networkMonitor.isOnline())
                val imageUrl = mediaUrlResolver.resolveImageUrl(
                    loaded.product.primaryImageUrl,
                    loaded.product.primaryImageFileId,
                )
                ProductDetailUiState(
                    product = loaded.product,
                    priceLabel = formatProductPrice(
                        loaded.product.priceAmount,
                        loaded.product.priceCurrency,
                    ),
                    stockAvailable = loaded.stockAvailable,
                    stockUnavailable = isStockUnavailable(loaded.stockAvailable),
                    imageUrl = imageUrl,
                    loading = false,
                )
            }.onSuccess { _state.value = it }
                .onFailure { error ->
                    _state.value = ProductDetailUiState(
                        loading = false,
                        errorCode = mapErrorCode(error),
                    )
                }
        }
    }

    private fun mapErrorCode(error: Throwable): String = when {
        error is ApiException -> error.detail.code
        error.message == "PRODUCT_NOT_FOUND" -> "PRODUCT_NOT_FOUND"
        else -> "LOAD_FAILED"
    }
}
