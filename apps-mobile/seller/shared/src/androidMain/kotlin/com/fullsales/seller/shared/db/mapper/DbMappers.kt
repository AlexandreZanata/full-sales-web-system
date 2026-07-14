package com.fullsales.seller.shared.db.mapper

import com.fullsales.seller.shared.db.entity.CommerceEntity
import com.fullsales.seller.shared.db.entity.ProductEntity
import com.fullsales.seller.shared.db.entity.SaleEntity
import com.fullsales.seller.shared.db.entity.SaleLineEntity
import com.fullsales.seller.shared.db.entity.SaleWithLines
import com.fullsales.seller.shared.db.entity.SyncOutboxEntity
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import com.fullsales.seller.shared.model.SyncOutboxEntry

fun CommerceEntity.toModel() = Commerce(id, legalName, tradeName, active, cnpj)

fun ProductEntity.toModel() = Product(
    id, name, sku, priceAmount, priceCurrency, active, categoryId, categoryName, categorySlug,
    primaryImageUrl, primaryImageFileId, unitOfMeasure, description,
)

fun Commerce.toEntity() = CommerceEntity(id, legalName, tradeName, active, cnpj)

fun Product.toEntity() = ProductEntity(
    id, name, sku, priceAmount, priceCurrency, active, categoryId, categoryName, categorySlug,
    primaryImageUrl, primaryImageFileId, unitOfMeasure, description,
)

fun SaleWithLines.toModel(): LocalSale = LocalSale(
    localId = sale.localId,
    remoteId = sale.remoteId,
    idempotencyKey = sale.idempotencyKey,
    commerceId = sale.commerceId,
    paymentMethod = sale.paymentMethod,
    status = LocalSaleStatus.valueOf(sale.status),
    totalAmount = sale.totalAmount,
    totalCurrency = sale.totalCurrency,
    items = lines.map {
        SaleItem(
            productId = it.productId,
            quantity = it.quantity,
            unitPriceAmount = it.unitPriceAmount,
            unitPriceCurrency = it.unitPriceCurrency,
            lineTotalAmount = it.lineTotalAmount,
        )
    },
    createdAtEpochMs = sale.createdAtEpochMs,
    syncFailureReason = sale.syncFailureReason,
    driverId = sale.driverId,
    origin = runCatching { SaleOrigin.valueOf(sale.origin) }.getOrDefault(SaleOrigin.Local),
)

fun SyncOutboxEntity.toModel() = SyncOutboxEntry(
    id, saleLocalId, method, path, bodyJson, idempotencyKey,
    createdAtEpochMs, attempts, lastError, completed, entityType,
)

fun SyncOutboxEntry.toEntity() = SyncOutboxEntity(
    id, saleLocalId, method, path, bodyJson, idempotencyKey,
    createdAtEpochMs, attempts, lastError, completed, entityType,
)

fun saleEntity(
    localId: String,
    idempotencyKey: String,
    commerceId: String,
    paymentMethod: String,
    totalAmount: Double,
    status: LocalSaleStatus,
    createdAtEpochMs: Long,
    driverId: String? = null,
    origin: SaleOrigin = SaleOrigin.Local,
) = SaleEntity(
    localId = localId,
    remoteId = null,
    idempotencyKey = idempotencyKey,
    commerceId = commerceId,
    paymentMethod = paymentMethod,
    status = status.name,
    totalAmount = totalAmount,
    totalCurrency = "BRL",
    createdAtEpochMs = createdAtEpochMs,
    driverId = driverId,
    origin = origin.name,
)

fun saleLines(localId: String, items: List<SaleItem>) =
    items.map {
        SaleLineEntity(
            saleLocalId = localId,
            productId = it.productId,
            quantity = it.quantity,
            unitPriceAmount = it.unitPriceAmount,
            unitPriceCurrency = it.unitPriceCurrency,
            lineTotalAmount = it.lineTotalAmount,
        )
    }
