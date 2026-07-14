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
    stockByProductId: Map<String, Int> = emptyMap(),
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
            else -> null
        }
        CreateSaleLineErrors(quantityError = quantityError)
    }
    return CreateSaleFormErrors(commerceError, paymentError, linesError, lineErrors)
}

/** True when sale line quantity exceeds known warehouse stock (confirm still allowed). */
fun saleLineNeedsBackorderWarning(
    productId: String,
    quantity: Int,
    stockByProductId: Map<String, Int>,
): Boolean {
    if (productId.isBlank() || quantity <= 0) return false
    val available = stockByProductId[productId] ?: return false
    return quantity > available
}

/** True when requested quantity exceeds known warehouse stock (sale still allowed). */
fun needsBackorderWarning(
    productId: String,
    lines: List<CreateSaleLineInput>,
    stockByProductId: Map<String, Int>,
): Boolean {
    if (productId.isBlank()) return false
    val available = stockByProductId[productId] ?: return false
    return reservedQuantityInSale(lines, productId) > available
}

/** All catalog products stay browsable; zero stock uses the existing backorder warning path. */
fun isProductAvailableForBrowsing(
    @Suppress("UNUSED_PARAMETER") stockByProductId: Map<String, Int>,
    @Suppress("UNUSED_PARAMETER") productId: String,
): Boolean = true

fun filterProductsAvailableForBrowsing(
    products: List<Product>,
    stockByProductId: Map<String, Int>,
): List<Product> = products.filter { isProductAvailableForBrowsing(stockByProductId, it.id) }

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

/** Quantity reserved in the open sale form for one product (all lines). */
fun reservedQuantityInSale(lines: List<CreateSaleLineInput>, productId: String): Int =
    lines.filter { it.productId == productId }
        .sumOf { it.quantityText.toIntOrNull() ?: 0 }

/**
 * Stock badge value while composing a sale. Decreases as lines are added; backend balance
 * is unchanged until the sale is submitted.
 */
fun visualStockRemaining(
    availableStock: Int?,
    lines: List<CreateSaleLineInput>,
    productId: String,
): Int? {
    if (availableStock == null || productId.isBlank()) return availableStock
    return (availableStock - reservedQuantityInSale(lines, productId)).coerceAtLeast(0)
}
