package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SalesListItem

fun mergeSalesList(
    remote: List<Sale>,
    local: List<LocalSale>,
    parseCreatedAt: (String?) -> Long = ::parseIso8601EpochMs,
): List<SalesListItem> {
    val remoteById = remote.associateBy { it.id }
    val items = mutableListOf<SalesListItem>()

    local.forEach { sale ->
        val remoteId = sale.remoteId
        when {
            remoteId != null && remoteById.containsKey(remoteId) -> Unit
            isLocalOnlySale(sale.status, remoteId) -> items.add(sale.toListItem(parseCreatedAt))
            remoteId != null -> items.add(sale.toListItem(parseCreatedAt))
        }
    }
    remote.forEach { sale -> items.add(sale.toListItem(parseCreatedAt)) }

    return items.sortedWith(
        compareBy<SalesListItem> { saleDisplaySortRank(it.status) }
            .thenByDescending { it.createdAtEpochMs },
    )
}

private fun LocalSale.toListItem(parseCreatedAt: (String?) -> Long): SalesListItem =
    SalesListItem(
        navigationId = remoteId ?: localId,
        shortId = (remoteId ?: localId).take(8),
        createdAtEpochMs = createdAtEpochMs,
        status = localSaleStatusToDisplay(status),
        totalAmount = totalAmount,
        totalCurrency = totalCurrency,
    )

private fun Sale.toListItem(parseCreatedAt: (String?) -> Long): SalesListItem =
    SalesListItem(
        navigationId = id,
        shortId = id.take(8),
        createdAtEpochMs = parseCreatedAt(createdAt),
        status = remoteSaleStatusToDisplay(status),
        totalAmount = totalAmount,
        totalCurrency = totalCurrency,
    )

internal expect fun parseIso8601EpochMs(iso: String?): Long
