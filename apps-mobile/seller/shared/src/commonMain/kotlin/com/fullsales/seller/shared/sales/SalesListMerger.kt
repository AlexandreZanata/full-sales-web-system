package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SalesListItem

/**
 * LocalStore-only list mapping (Phase 16B). Prefer over [mergeSalesList] for UI.
 */
fun localSalesToListItems(local: List<LocalSale>): List<SalesListItem> =
    local.map { it.toListItem() }
        .sortedWith(
            compareBy<SalesListItem> { saleDisplaySortRank(it.status) }
                .thenByDescending { it.createdAtEpochMs },
        )

@Deprecated("Prefer localSalesToListItems — remote RAM merge removed in 16B")
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
            isLocalOnlySale(sale.status, remoteId) -> items.add(sale.toListItem())
            remoteId != null -> items.add(sale.toListItem())
        }
    }
    remote.forEach { sale -> items.add(sale.toRemoteListItem(parseCreatedAt)) }

    return items.sortedWith(
        compareBy<SalesListItem> { saleDisplaySortRank(it.status) }
            .thenByDescending { it.createdAtEpochMs },
    )
}

private fun LocalSale.toListItem(): SalesListItem =
    SalesListItem(
        navigationId = remoteId ?: localId,
        shortId = (remoteId ?: localId).take(8),
        createdAtEpochMs = createdAtEpochMs,
        status = localSaleStatusToDisplay(status),
        totalAmount = totalAmount,
        totalCurrency = totalCurrency,
    )

private fun Sale.toRemoteListItem(parseCreatedAt: (String?) -> Long): SalesListItem =
    SalesListItem(
        navigationId = id,
        shortId = id.take(8),
        createdAtEpochMs = parseCreatedAt(createdAt),
        status = remoteSaleStatusToDisplay(status),
        totalAmount = totalAmount,
        totalCurrency = totalCurrency,
    )

internal expect fun parseIso8601EpochMs(iso: String?): Long
