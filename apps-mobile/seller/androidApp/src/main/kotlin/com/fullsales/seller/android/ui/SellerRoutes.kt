package com.fullsales.seller.android.ui

object SellerRoutes {
    const val LOGIN = "login"
    const val SALES = "sales"
    const val SALES_NEW = "sales/new"
    const val SALE_DETAIL = "sales/{saleId}"
    const val COMMERCES = "commerces"
    const val COMMERCE_PICK = "commerces/pick"
    const val COMMERCE_DETAIL = "commerces/{commerceId}"
    const val PRODUCTS = "products"
    const val PRODUCT_DETAIL = "products/{productId}"

    fun saleDetail(saleId: String) = "sales/$saleId"
    fun commerceDetail(commerceId: String) = "commerces/$commerceId"
    fun productDetail(productId: String) = "products/$productId"

    fun showsBottomBar(route: String?): Boolean =
        route == SALES || route == SALES_NEW
}
