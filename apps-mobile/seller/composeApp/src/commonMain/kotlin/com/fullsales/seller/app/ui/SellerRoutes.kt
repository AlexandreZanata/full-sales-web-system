package com.fullsales.seller.app.ui

object SellerRoutes {
    const val LOGIN = "login"
    const val OFFLINE = "offline"
    const val PROFILE = "profile"
    const val SALES = "sales"
    const val SALES_NEW = "sales/new"
    const val SALE_DETAIL = "sales/{saleId}"
    const val COMMERCES = "commerces"
    const val COMMERCE_PICK = "commerces/pick"
    const val COMMERCE_DETAIL = "commerces/{commerceId}"
    const val COMMERCE_REGISTRATIONS = "commerces/registrations"
    const val COMMERCE_REGISTRATION_MODE = "commerces/registrations/mode"
    const val COMMERCE_REGISTRATION_CNPJ = "commerces/registrations/cnpj"
    const val COMMERCE_REGISTRATION_FORM = "commerces/registrations/form"
    const val PRODUCTS = "products"
    const val PRODUCT_DETAIL = "products/{productId}"

    fun saleDetail(saleId: String) = "sales/$saleId"
    fun commerceDetail(commerceId: String) = "commerces/$commerceId"
    fun productDetail(productId: String) = "products/$productId"

    fun isCommerceRegistrationFlow(route: String?): Boolean =
        route == COMMERCE_REGISTRATIONS ||
            route == COMMERCE_REGISTRATION_MODE ||
            route == COMMERCE_REGISTRATION_CNPJ ||
            route == COMMERCE_REGISTRATION_FORM

    fun isCommerceTabSelected(route: String?): Boolean =
        route == COMMERCES || isCommerceRegistrationFlow(route)

    fun showsBottomBar(route: String?): Boolean =
        route == SALES || route == COMMERCES || isCommerceRegistrationFlow(route)
}
