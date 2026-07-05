package com.fullsales.seller.app.ui.products

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
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
    private val apiClient: SellerApiClient,
    private val mediaUrlResolver: MediaUrlResolver,
) : ViewModel() {
    private val _state = MutableStateFlow(ProductDetailUiState())
    val state: StateFlow<ProductDetailUiState> = _state.asStateFlow()

    fun load(productId: String) {
        viewModelScope.launch {
            _state.value = ProductDetailUiState(loading = true)
            runCatching {
                val product = apiClient.getProduct(productId)
                val balance = runCatching { apiClient.getStockBalance(productId) }.getOrNull()
                val imageUrl = mediaUrlResolver.resolveImageUrl(
                    product.primaryImageUrl,
                    product.primaryImageFileId,
                )
                ProductDetailUiState(
                    product = product,
                    priceLabel = formatProductPrice(product.priceAmount, product.priceCurrency),
                    stockAvailable = balance?.available,
                    stockUnavailable = isStockUnavailable(balance?.available),
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

    private fun mapErrorCode(error: Throwable): String = when (error) {
        is ApiException -> error.detail.code
        else -> "LOAD_FAILED"
    }
}
