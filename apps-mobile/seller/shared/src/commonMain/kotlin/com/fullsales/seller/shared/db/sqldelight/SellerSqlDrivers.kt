package com.fullsales.seller.shared.db.sqldelight

import app.cash.sqldelight.db.SqlDriver

fun createSellerLocalDatabase(driver: SqlDriver): SellerLocalDatabase =
    SellerLocalDatabase(driver)

fun createSellerLocalSchema(driver: SqlDriver) {
    SellerLocalDatabase.Schema.create(driver)
}
