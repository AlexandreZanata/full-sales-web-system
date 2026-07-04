package com.fullsales.field.shared.db.mapper

import com.fullsales.field.shared.db.entity.CommerceEntity
import com.fullsales.field.shared.db.entity.ProductEntity
import com.fullsales.field.shared.db.entity.SaleEntity
import com.fullsales.field.shared.db.entity.SaleLineEntity
import com.fullsales.field.shared.db.entity.SaleWithLines
import com.fullsales.field.shared.db.entity.StockBalanceEntity
import com.fullsales.field.shared.db.entity.SyncOutboxEntity
import com.fullsales.field.shared.model.Commerce
import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.model.Product
import com.fullsales.field.shared.model.Sale
import com.fullsales.field.shared.model.SaleItem
import com.fullsales.field.shared.model.StockBalance
import com.fullsales.field.shared.model.SyncOutboxEntry

fun CommerceEntity.toModel() = Commerce(id, legalName, tradeName, active)
fun ProductEntity.toModel() = Product(id, name, sku, priceAmount, priceCurrency, active)
fun StockBalanceEntity.toModel() = StockBalance(productId, available, asOf)

fun Commerce.toEntity() = CommerceEntity(id, legalName, tradeName, active)
fun Product.toEntity() = ProductEntity(id, name, sku, priceAmount, priceCurrency, active)
fun StockBalance.toEntity() = StockBalanceEntity(productId, available, asOf)

fun SaleWithLines.toModel(): Sale = Sale(
    localId = sale.localId,
    remoteId = sale.remoteId,
    commerceId = sale.commerceId,
    status = LocalSaleStatus.valueOf(sale.status),
    paymentMethod = sale.paymentMethod,
    totalAmount = sale.totalAmount,
    totalCurrency = sale.totalCurrency,
    items = lines.map { SaleItem(it.productId, it.quantity) },
    createdAtEpochMs = sale.createdAtEpochMs,
)

fun SyncOutboxEntity.toModel() = SyncOutboxEntry(
    id, saleLocalId, method, path, bodyJson, idempotencyKey,
    createdAtEpochMs, attempts, lastError, completed,
)

fun SyncOutboxEntry.toEntity() = SyncOutboxEntity(
    id, saleLocalId, method, path, bodyJson, idempotencyKey,
    createdAtEpochMs, attempts, lastError, completed,
)

fun saleEntity(
    localId: String,
    commerceId: String,
    paymentMethod: String,
    totalAmount: Double,
    status: LocalSaleStatus,
    createdAtEpochMs: Long,
) = SaleEntity(
    localId = localId,
    remoteId = null,
    commerceId = commerceId,
    paymentMethod = paymentMethod,
    status = status.name,
    totalAmount = totalAmount,
    totalCurrency = "BRL",
    createdAtEpochMs = createdAtEpochMs,
)

fun saleLines(localId: String, items: List<SaleItem>) =
    items.map { SaleLineEntity(saleLocalId = localId, productId = it.productId, quantity = it.quantity) }
