package com.fullsales.seller.shared.db.sqldelight

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.LocalRegistration
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import com.fullsales.seller.shared.model.SyncOutboxEntry

fun Commerces.toModel() = Commerce(id, legalName, tradeName, active, cnpj)

fun Products.toModel() = Product(
    id, name, sku, priceAmount, priceCurrency, active, categoryId, categoryName, categorySlug,
    primaryImageUrl, primaryImageFileId, unitOfMeasure, description,
)

fun Sales.toLocalSale(lines: List<Sale_lines>): LocalSale = LocalSale(
    localId = localId,
    remoteId = remoteId,
    idempotencyKey = idempotencyKey,
    commerceId = commerceId,
    paymentMethod = paymentMethod,
    status = LocalSaleStatus.valueOf(status),
    totalAmount = totalAmount,
    totalCurrency = totalCurrency,
    items = lines.map { it.toSaleItem() },
    createdAtEpochMs = createdAtEpochMs,
    syncFailureReason = syncFailureReason,
    driverId = driverId,
    origin = runCatching { SaleOrigin.valueOf(origin) }.getOrDefault(SaleOrigin.Local),
)

fun Sale_lines.toSaleItem() = SaleItem(
    productId = productId,
    quantity = quantity.toInt(),
    unitPriceAmount = unitPriceAmount ?: 0.0,
    unitPriceCurrency = unitPriceCurrency ?: "BRL",
    lineTotalAmount = lineTotalAmount ?: 0.0,
)

fun Sync_outbox.toModel() = SyncOutboxEntry(
    id = id,
    aggregateId = aggregateId,
    method = method,
    path = path,
    bodyJson = bodyJson,
    idempotencyKey = idempotencyKey,
    createdAtEpochMs = createdAtEpochMs,
    attempts = attempts.toInt(),
    lastError = lastError,
    completed = completed,
    entityType = entityType,
    dependsOnOutboxId = dependsOnOutboxId,
)

fun Registrations.toModel() = LocalRegistration(
    localId = localId,
    remoteId = remoteId,
    cnpj = cnpj,
    legalName = legalName,
    tradeName = tradeName,
    active = active,
    registrationStatus = registrationStatus,
    rejectionReason = rejectionReason,
    registrationMode = registrationMode.takeIf { it.isNotBlank() },
    contactPhone = contactPhone,
    contactEmail = contactEmail,
    deliveryAddressJson = deliveryAddressJson,
    syncStatus = runCatching { LocalRegistrationSyncStatus.valueOf(syncStatus) }
        .getOrDefault(LocalRegistrationSyncStatus.Synced),
    syncFailureReason = syncFailureReason,
    createdAtEpochMs = createdAtEpochMs,
    updatedAtEpochMs = updatedAtEpochMs,
    idempotencyKey = idempotencyKey,
)

internal fun CatalogQueries.upsertCommerceRow(c: Commerce) {
    upsertCommerce(c.id, c.legalName, c.tradeName, c.active, c.cnpj)
}

internal fun CatalogQueries.upsertProductRow(p: Product) {
    upsertProduct(
        p.id, p.name, p.sku, p.priceAmount, p.priceCurrency, p.active,
        p.categoryId, p.categoryName, p.categorySlug, p.primaryImageUrl, p.primaryImageFileId,
        p.unitOfMeasure, p.description,
    )
}

internal fun SalesQueries.upsertSaleRow(sale: LocalSale) {
    upsertSale(
        localId = sale.localId,
        remoteId = sale.remoteId,
        idempotencyKey = sale.idempotencyKey,
        commerceId = sale.commerceId,
        paymentMethod = sale.paymentMethod,
        status = sale.status.name,
        totalAmount = sale.totalAmount,
        totalCurrency = sale.totalCurrency,
        createdAtEpochMs = sale.createdAtEpochMs,
        syncFailureReason = sale.syncFailureReason,
        driverId = sale.driverId,
        origin = sale.origin.name,
    )
}

internal fun SalesQueries.insertSaleItems(saleLocalId: String, items: List<SaleItem>) {
    items.forEach { item ->
        insertSaleLine(
            saleLocalId = saleLocalId,
            productId = item.productId,
            quantity = item.quantity.toLong(),
            unitPriceAmount = item.unitPriceAmount,
            unitPriceCurrency = item.unitPriceCurrency,
            lineTotalAmount = item.lineTotalAmount,
        )
    }
}

internal fun RegistrationsQueries.upsertRegistrationRow(row: LocalRegistration) {
    upsertRegistration(
        localId = row.localId,
        remoteId = row.remoteId,
        cnpj = row.cnpj,
        legalName = row.legalName,
        tradeName = row.tradeName,
        active = row.active,
        registrationStatus = row.registrationStatus,
        rejectionReason = row.rejectionReason,
        registrationMode = row.registrationMode.orEmpty(),
        contactPhone = row.contactPhone,
        contactEmail = row.contactEmail,
        deliveryAddressJson = row.deliveryAddressJson,
        syncStatus = row.syncStatus.name,
        syncFailureReason = row.syncFailureReason,
        createdAtEpochMs = row.createdAtEpochMs,
        updatedAtEpochMs = row.updatedAtEpochMs,
        idempotencyKey = row.idempotencyKey,
    )
}

internal fun OutboxQueries.insertOutboxRow(entry: SyncOutboxEntry) {
    insertOutbox(
        id = entry.id,
        aggregateId = entry.aggregateId,
        method = entry.method,
        path = entry.path,
        bodyJson = entry.bodyJson,
        idempotencyKey = entry.idempotencyKey,
        createdAtEpochMs = entry.createdAtEpochMs,
        attempts = entry.attempts.toLong(),
        lastError = entry.lastError,
        completed = entry.completed,
        entityType = entry.entityType,
        dependsOnOutboxId = entry.dependsOnOutboxId,
    )
}
