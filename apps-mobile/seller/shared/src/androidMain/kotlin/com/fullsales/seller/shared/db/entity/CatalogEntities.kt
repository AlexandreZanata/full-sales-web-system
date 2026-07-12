package com.fullsales.seller.shared.db.entity

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
    val categoryId: String? = null,
    val categoryName: String? = null,
    val categorySlug: String? = null,
    val primaryImageUrl: String? = null,
    val primaryImageFileId: String? = null,
)

@Entity(tableName = "stock_snapshots")
data class StockSnapshotEntity(
    @PrimaryKey val productId: String,
    val available: Int,
    val syncedAtEpochMs: Long,
)

@Entity(tableName = "commerce_address_cache")
data class CommerceAddressCacheEntity(
    @PrimaryKey val commerceId: String,
    val addressesJson: String,
    val syncedAtEpochMs: Long,
)
