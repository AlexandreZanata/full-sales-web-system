package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.i18n.CreateSaleValidationError
import com.fullsales.seller.shared.model.Product
import kotlinx.serialization.Serializable

val PAYMENT_METHODS = listOf("cash", "pix", "credit", "debit")

/** Sum line totals in minor units — matches field PWA `/sales/new`. */
fun calculateCreateSaleTotalMinor(
    products: List<Product>,
    lines: List<CreateSaleLineInput>,
): Long = lines.sumOf { line ->
    val product = products.firstOrNull { it.id == line.productId } ?: return@sumOf 0L
    val qty = line.quantityText.toIntOrNull() ?: return@sumOf 0L
    if (qty <= 0) return@sumOf 0L
    (product.priceAmount * qty).toLong()
}

@Serializable
data class CreateSaleLineInput(
    val productId: String = "",
    val quantityText: String = "1",
    val productSearchQuery: String = "",
)

data class CreateSaleLineErrors(
    val quantityError: CreateSaleValidationError? = null,
)

data class CreateSaleFormErrors(
    val commerceError: CreateSaleValidationError? = null,
    val paymentError: CreateSaleValidationError? = null,
    val linesError: CreateSaleValidationError? = null,
    val lineErrors: List<CreateSaleLineErrors> = emptyList(),
) {
    val isValid: Boolean
        get() = commerceError == null && paymentError == null && linesError == null &&
            lineErrors.all { it.quantityError == null }
}

fun validateCreateSaleForm(
    commerceId: String,
    paymentMethod: String,
    lines: List<CreateSaleLineInput>,
    stockByProductId: Map<String, Int>,
): CreateSaleFormErrors {
    val commerceError = if (commerceId.isBlank()) CreateSaleValidationError.SelectCommerce else null
    val paymentError = if (paymentMethod.isBlank()) CreateSaleValidationError.SelectPayment else null
    val validLineCount = lines.count { line ->
        line.productId.isNotBlank() && (line.quantityText.toIntOrNull() ?: 0) > 0
    }
    val linesError = if (validLineCount == 0) CreateSaleValidationError.AddProductLine else null
    val lineErrors = lines.map { line ->
        val qty = line.quantityText.toIntOrNull()
        val quantityError = when {
            line.productId.isBlank() -> null
            qty == null || qty <= 0 -> CreateSaleValidationError.QuantityRequired
            else -> {
                val stock = stockByProductId[line.productId]
                if (stock != null && qty > stock) {
                    CreateSaleValidationError.QuantityExceedsStock(stock)
                } else {
                    null
                }
            }
        }
        CreateSaleLineErrors(quantityError = quantityError)
    }
    return CreateSaleFormErrors(commerceError, paymentError, linesError, lineErrors)
}

fun buildCreateSaleRequest(
    commerceId: String,
    paymentMethod: String,
    lines: List<CreateSaleLineInput>,
): com.fullsales.seller.shared.model.CreateSaleRequest {
    val items = lines
        .filter { it.productId.isNotBlank() }
        .mapNotNull { line ->
            val qty = line.quantityText.toIntOrNull() ?: return@mapNotNull null
            if (qty <= 0) return@mapNotNull null
            com.fullsales.seller.shared.model.CreateSaleItem(line.productId, qty)
        }
    return com.fullsales.seller.shared.model.CreateSaleRequest(
        commerceId = commerceId,
        paymentMethod = paymentMethod,
        items = items,
    )
}
