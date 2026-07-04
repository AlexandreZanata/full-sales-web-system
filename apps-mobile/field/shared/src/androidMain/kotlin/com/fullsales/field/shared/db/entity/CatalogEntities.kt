package com.fullsales.field.shared.db.entity

import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "commerces")
data class CommerceEntity(
    @PrimaryKey val id: String,
    val legalName: String,
    val tradeName: String?,
    val active: Boolean,
)

@Entity(tableName = "products")
data class ProductEntity(
    @PrimaryKey val id: String,
    val name: String,
    val sku: String,
    val priceAmount: Double,
    val priceCurrency: String,
    val active: Boolean,
)

@Entity(tableName = "stock_balances")
data class StockBalanceEntity(
    @PrimaryKey val productId: String,
    val available: Int,
    val asOf: String,
)

@Entity(tableName = "sync_metadata")
data class SyncMetadataEntity(
    @PrimaryKey val key: String,
    val value: String,
)
