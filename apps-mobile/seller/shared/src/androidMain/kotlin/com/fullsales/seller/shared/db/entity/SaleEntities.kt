package com.fullsales.seller.shared.db.entity

import androidx.room.Entity
import androidx.room.ForeignKey
import androidx.room.Index
import androidx.room.PrimaryKey

@Entity(
    tableName = "sales",
    indices = [Index("remoteId"), Index("idempotencyKey")],
)
data class SaleEntity(
    @PrimaryKey val localId: String,
    val remoteId: String?,
    val idempotencyKey: String,
    val commerceId: String,
    val paymentMethod: String,
    val status: String,
    val totalAmount: Double,
    val totalCurrency: String,
    val createdAtEpochMs: Long,
    val syncFailureReason: String? = null,
    val driverId: String? = null,
    val origin: String = "Local",
)

@Entity(
    tableName = "sale_lines",
    foreignKeys = [
        ForeignKey(
            entity = SaleEntity::class,
            parentColumns = ["localId"],
            childColumns = ["saleLocalId"],
            onDelete = ForeignKey.CASCADE,
        ),
    ],
    indices = [Index("saleLocalId")],
)
data class SaleLineEntity(
    @PrimaryKey(autoGenerate = true) val id: Long = 0,
    val saleLocalId: String,
    val productId: String,
    val quantity: Int,
    val unitPriceAmount: Double = 0.0,
    val unitPriceCurrency: String = "BRL",
    val lineTotalAmount: Double = 0.0,
)

@Entity(
    tableName = "sync_outbox",
    indices = [Index("saleLocalId"), Index("completed")],
)
data class SyncOutboxEntity(
    @PrimaryKey val id: String,
    val saleLocalId: String,
    val method: String,
    val path: String,
    val bodyJson: String,
    val idempotencyKey: String,
    val createdAtEpochMs: Long,
    val attempts: Int,
    val lastError: String?,
    val completed: Boolean,
    val entityType: String = "Sale",
)
