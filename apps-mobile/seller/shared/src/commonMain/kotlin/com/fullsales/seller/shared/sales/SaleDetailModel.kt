package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.i18n.SyncChipStatus
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleDisplayStatus
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.displayName

data class SaleDetailLine(
    val productId: String,
    val productLabel: String,
    val quantity: Int,
    val lineTotalMinor: Double,
    val currency: String,
)

data class SaleDetailModel(
    val navigationId: String,
    val localId: String?,
    val remoteId: String?,
    val commerceId: String,
    val commerceName: String?,
    val paymentMethod: String,
    val status: SaleDisplayStatus,
    val syncChip: SyncChipStatus?,
    val totalAmountMinor: Double,
    val totalCurrency: String,
    val items: List<SaleDetailLine>,
) {
    val showActions: Boolean = showSaleDetailActions(status, remoteId)
}

fun showSaleDetailActions(status: SaleDisplayStatus, remoteId: String?): Boolean =
    status == SaleDisplayStatus.Pending && remoteId != null

fun syncChipStatus(status: LocalSaleStatus): SyncChipStatus? = when (status) {
    LocalSaleStatus.PendingSync -> SyncChipStatus.PendingSync
    LocalSaleStatus.SyncFailed -> SyncChipStatus.SyncFailed
    else -> null
}

fun buildSaleDetailFromRemote(
    sale: Sale,
    local: LocalSale?,
    commerces: List<Commerce>,
    products: List<Product>,
): SaleDetailModel = SaleDetailModel(
    navigationId = sale.id,
    localId = local?.localId,
    remoteId = sale.id,
    commerceId = sale.commerceId,
    commerceName = commerces.firstOrNull { it.id == sale.commerceId }?.displayName(),
    paymentMethod = sale.paymentMethod,
    status = remoteSaleStatusToDisplay(sale.status),
    syncChip = local?.let { syncChipStatus(it.status) },
    totalAmountMinor = sale.totalAmount,
    totalCurrency = sale.totalCurrency,
    items = sale.items.map { it.toDetailLine(products) },
)

fun buildSaleDetailFromLocal(
    local: LocalSale,
    commerces: List<Commerce>,
    products: List<Product>,
): SaleDetailModel {
    val displayStatus = when (local.status) {
        LocalSaleStatus.Confirmed -> SaleDisplayStatus.Confirmed
        LocalSaleStatus.Cancelled -> SaleDisplayStatus.Cancelled
        LocalSaleStatus.SyncFailed -> SaleDisplayStatus.SyncFailed
        LocalSaleStatus.PendingSync, LocalSaleStatus.Draft -> SaleDisplayStatus.PendingSync
        LocalSaleStatus.Synced -> SaleDisplayStatus.Pending
    }
    return SaleDetailModel(
        navigationId = local.remoteId ?: local.localId,
        localId = local.localId,
        remoteId = local.remoteId,
        commerceId = local.commerceId,
        commerceName = commerces.firstOrNull { it.id == local.commerceId }?.displayName(),
        paymentMethod = local.paymentMethod,
        status = displayStatus,
        syncChip = syncChipStatus(local.status),
        totalAmountMinor = local.totalAmount,
        totalCurrency = local.totalCurrency,
        items = local.items.map { it.toDetailLine(products) },
    )
}

private fun SaleItem.toDetailLine(products: List<Product>): SaleDetailLine {
    val product = products.firstOrNull { it.id == productId }
    val label = product?.let { "${it.name} (${it.sku})" } ?: productId.take(8)
    val lineTotal = if (lineTotalAmount > 0) lineTotalAmount else unitPriceAmount * quantity
    return SaleDetailLine(
        productId = productId,
        productLabel = label,
        quantity = quantity,
        lineTotalMinor = lineTotal,
        currency = unitPriceCurrency.ifBlank { "BRL" },
    )
}
