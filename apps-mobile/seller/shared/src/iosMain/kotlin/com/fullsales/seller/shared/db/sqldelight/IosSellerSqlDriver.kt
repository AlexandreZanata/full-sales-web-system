package com.fullsales.seller.shared.db.sqldelight

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.native.NativeSqliteDriver

fun createIosSellerSqlDriver(): SqlDriver =
    NativeSqliteDriver(SellerLocalDatabase.Schema, "seller.db")
