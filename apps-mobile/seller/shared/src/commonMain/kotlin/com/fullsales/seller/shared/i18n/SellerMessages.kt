package com.fullsales.seller.shared.i18n

data class SellerMessages(
    val nav: Nav = Nav(),
    val auth: Auth = Auth(),
    val common: Common = Common(),
    val sales: Sales = Sales(),
    val status: Status = Status(),
    val syncStatus: SyncStatus = SyncStatus(),
    val paymentMethods: PaymentMethods = PaymentMethods(),
    val commerces: Commerces = Commerces(),
    val products: Products = Products(),
    val a11y: A11y = A11y(),
) {
    data class Nav(
        val sales: String = "",
        val newSale: String = "",
        val logout: String = "",
        val sellerFallback: String = "",
    )

    data class Auth(
        val signInTitle: String = "",
        val signIn: String = "",
        val signingIn: String = "",
        val email: String = "",
        val password: String = "",
        val sellerRequired: String = "",
        val invalidCredentials: String = "",
        val rateLimited: String = "",
        val invalidSession: String = "",
        val loginFailed: String = "",
        val language: String = "",
    )

    data class Common(
        val cancel: String = "",
        val confirm: String = "",
        val back: String = "",
        val working: String = "",
        val saving: String = "",
        val tryAgain: String = "",
        val loading: String = "",
        val loadFailed: String = "",
        val offline: String = "",
        val syncing: String = "",
        val syncFailed: String = "",
        val search: String = "",
        val quantity: String = "",
        val total: String = "",
        val stockAvailable: String = "",
        val stockUnknown: String = "",
        val stockUnavailable: String = "",
        val active: String = "",
        val inactive: String = "",
        val all: String = "",
        val primary: String = "",
    )

    data class Sales(
        val title: String = "",
        val new: String = "",
        val detail: String = "",
        val confirm: String = "",
        val confirmShort: String = "",
        val cancel: String = "",
        val cancelShort: String = "",
        val commerce: String = "",
        val browseCommerces: String = "",
        val paymentMethod: String = "",
        val paymentLabel: String = "",
        val addLine: String = "",
        val lineItem: String = "",
        val product: String = "",
        val selectCommerce: String = "",
        val selectPayment: String = "",
        val addProductLine: String = "",
        val topSelling: String = "",
        val noProductResults: String = "",
        val quantityRequired: String = "",
        val created: String = "",
        val confirmed: String = "",
        val cancelled: String = "",
        val emptyTitle: String = "",
        val emptyMessage: String = "",
        val emptyAction: String = "",
        val offlineTitle: String = "",
        val offlineMessage: String = "",
        val loadErrorTitle: String = "",
        val loadErrorOffline: String = "",
        val loadFailed: String = "",
        val actionFailed: String = "",
        val insufficientStock: String = "",
        val invalidTransition: String = "",
        val notFound: String = "",
        val createFailed: String = "",
        val createValidation: String = "",
        val commerceNotFound: String = "",
        val saveOfflineFailed: String = "",
        val networkError: String = "",
    )

    data class Status(
        val pending: String = "",
        val confirmed: String = "",
        val cancelled: String = "",
        val pendingSync: String = "",
        val syncFailed: String = "",
    )

    data class SyncStatus(
        val pendingSync: String = "",
        val syncFailed: String = "",
    )

    data class PaymentMethods(
        val cash: String = "",
        val pix: String = "",
        val credit: String = "",
        val debit: String = "",
    )

    data class Commerces(
        val title: String = "",
        val selectTitle: String = "",
        val searchByName: String = "",
        val empty: String = "",
        val emptyOffline: String = "",
        val addresses: String = "",
        val noAddresses: String = "",
        val cnpjLabel: String = "",
    )

    data class Products(
        val title: String = "",
        val searchByNameOrSku: String = "",
        val empty: String = "",
        val skuLabel: String = "",
        val categoryLabel: String = "",
        val unitLabel: String = "",
        val addToSale: String = "",
        val notFound: String = "",
        val sessionExpired: String = "",
        val loadFailed: String = "",
    )

    data class A11y(
        val menu: String = "",
        val newSale: String = "",
        val sales: String = "",
        val removeLine: String = "",
        val addLine: String = "",
        val language: String = "",
        val textSizeLabel: String = "",
        val textSizeNormal: String = "",
        val textSizeLarge: String = "",
        val textSizeExtraLarge: String = "",
        val selected: String = "",
        val saleListItem: String = "",
        val commerceListItem: String = "",
        val productListItem: String = "",
        val pullToRefresh: String = "",
    )
}
