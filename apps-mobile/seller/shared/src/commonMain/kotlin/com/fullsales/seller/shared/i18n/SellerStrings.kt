package com.fullsales.seller.shared.i18n

import com.fullsales.seller.shared.model.SaleDisplayStatus

object SellerStrings {
    fun forLocale(locale: SellerLocale): SellerMessages = when (locale) {
        SellerLocale.En -> messagesEn
        SellerLocale.PtBr -> messagesPtBr
    }

    fun saleStatus(messages: SellerMessages, status: SaleDisplayStatus): String = when (status) {
        SaleDisplayStatus.Pending -> messages.status.pending
        SaleDisplayStatus.Confirmed -> messages.status.confirmed
        SaleDisplayStatus.Cancelled -> messages.status.cancelled
        SaleDisplayStatus.PendingSync -> messages.status.pendingSync
        SaleDisplayStatus.SyncFailed -> messages.status.syncFailed
    }

    fun paymentMethod(messages: SellerMessages, method: String): String = when (method) {
        "cash" -> messages.paymentMethods.cash
        "pix" -> messages.paymentMethods.pix
        "credit" -> messages.paymentMethods.credit
        "debit" -> messages.paymentMethods.debit
        else -> method
    }

    fun stockBadge(messages: SellerMessages, available: Int?): String = when {
        available == null -> messages.common.stockUnknown
        available <= 0 -> messages.common.stockUnavailable
        else -> format(messages.common.stockAvailable, "qty" to available.toString())
    }

    fun saleActionError(messages: SellerMessages, code: String): String = when (code) {
        "INSUFFICIENT_STOCK" -> messages.sales.insufficientStock
        "INVALID_SALE_TRANSITION" -> messages.sales.invalidTransition
        "SALE_NOT_FOUND" -> messages.sales.notFound
        "NO_REMOTE_ID" -> messages.sales.loadErrorOffline
        "OFFLINE_UNAVAILABLE" -> messages.sales.loadErrorOffline
        "LOAD_FAILED" -> messages.sales.loadFailed
        else -> messages.sales.actionFailed
    }

    fun createSaleError(messages: SellerMessages, code: String): String = when (code) {
        "INSUFFICIENT_STOCK" -> messages.sales.insufficientStock
        "VALIDATION_ERROR" -> messages.sales.createValidation
        "COMMERCE_NOT_FOUND" -> messages.sales.commerceNotFound
        "LOCAL_ERROR" -> messages.sales.saveOfflineFailed
        "NETWORK_ERROR" -> messages.sales.networkError
        else -> messages.sales.createFailed
    }

    fun authError(messages: SellerMessages, code: AuthErrorCode, detail: String? = null): String =
        when (code) {
            AuthErrorCode.SELLER_REQUIRED -> messages.auth.sellerRequired
            AuthErrorCode.INVALID_CREDENTIALS -> messages.auth.invalidCredentials
            AuthErrorCode.RATE_LIMITED -> messages.auth.rateLimited
            AuthErrorCode.INVALID_SESSION -> messages.auth.invalidSession
            AuthErrorCode.LOGIN_FAILED -> messages.auth.loginFailed
            AuthErrorCode.GENERIC -> detail ?: messages.auth.loginFailed
        }

    fun productError(messages: SellerMessages, code: String): String = when (code) {
        "PRODUCT_NOT_FOUND" -> messages.products.notFound
        "UNAUTHORIZED" -> messages.products.sessionExpired
        "LOAD_FAILED" -> messages.products.loadFailed
        else -> messages.products.loadFailed
    }

    fun formatValidation(messages: SellerMessages, error: CreateSaleValidationError): String =
        when (error) {
            CreateSaleValidationError.SelectCommerce -> messages.sales.selectCommerce
            CreateSaleValidationError.SelectPayment -> messages.sales.selectPayment
            CreateSaleValidationError.AddProductLine -> messages.sales.addProductLine
            CreateSaleValidationError.QuantityRequired -> messages.sales.quantityRequired
            is CreateSaleValidationError.QuantityExceedsStock ->
                format(messages.common.stockAvailable, "qty" to error.available.toString())
        }

    fun format(template: String, vararg vars: Pair<String, String>): String {
        var result = template
        vars.forEach { (key, value) -> result = result.replace("{$key}", value) }
        return result
    }

    fun saleListItem(
        messages: SellerMessages,
        id: String,
        date: String,
        status: String,
        amount: String,
    ): String = format(
        messages.a11y.saleListItem,
        "id" to id,
        "date" to date,
        "status" to status,
        "amount" to amount,
    )

    fun commerceListItem(messages: SellerMessages, name: String, status: String): String =
        format(messages.a11y.commerceListItem, "name" to name, "status" to status)

    fun productListItem(messages: SellerMessages, name: String, sku: String, price: String): String =
        format(messages.a11y.productListItem, "name" to name, "sku" to sku, "price" to price)
}

enum class AuthErrorCode {
    SELLER_REQUIRED,
    INVALID_CREDENTIALS,
    RATE_LIMITED,
    INVALID_SESSION,
    LOGIN_FAILED,
    GENERIC,
}

sealed interface CreateSaleValidationError {
    data object SelectCommerce : CreateSaleValidationError
    data object SelectPayment : CreateSaleValidationError
    data object AddProductLine : CreateSaleValidationError
    data object QuantityRequired : CreateSaleValidationError
    data class QuantityExceedsStock(val available: Int) : CreateSaleValidationError
}

enum class SyncChipStatus {
    PendingSync,
    SyncFailed,
}
