package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SalesListItem

/**
 * LocalStore-only list mapping (Phase 16B). Prefer over [mergeSalesList] for UI.
 */
fun localSalesToListItems(local: List<LocalSale>): List<SalesListItem> {
    val codes = saleDisplayCodes(local)
    return local.map { it.toListItem(codes) }
        .sortedWith(
            compareBy<SalesListItem> { saleDisplaySortRank(it.status) }
                .thenByDescending { it.createdAtEpochMs },
        )
}

@Deprecated("Prefer localSalesToListItems — remote RAM merge removed in 16B")
fun mergeSalesList(
    remote: List<Sale>,
    local: List<LocalSale>,
    parseCreatedAt: (String?) -> Long = ::parseIso8601EpochMs,
): List<SalesListItem> {
    val remoteById = remote.associateBy { it.id }
    val items = mutableListOf<SalesListItem>()

    val codes = saleDisplayCodes(local).toMutableMap()
    var nextSequence = local.size + 1
    remote
        .sortedBy { parseCreatedAt(it.createdAt) }
        .forEach { sale ->
            if (sale.id !in codes) {
                codes[sale.id] = formatSaleDisplayCode(nextSequence++)
            }
        }
    local.forEach { sale ->
        val remoteId = sale.remoteId
        when {
            remoteId != null && remoteById.containsKey(remoteId) -> Unit
            isLocalOnlySale(sale.status, remoteId) -> items.add(sale.toListItem(codes))
            remoteId != null -> items.add(sale.toListItem(codes))
        }
    }
    remote.forEach { sale -> items.add(sale.toRemoteListItem(parseCreatedAt, codes)) }

    return items.sortedWith(
        compareBy<SalesListItem> { saleDisplaySortRank(it.status) }
            .thenByDescending { it.createdAtEpochMs },
    )
}

private fun LocalSale.toListItem(codes: Map<String, String>): SalesListItem {
    val navigationId = remoteId ?: localId
    return SalesListItem(
        navigationId = navigationId,
        shortId = displayCode
            ?: codes[navigationId]
            ?: codes[localId]
            ?: formatSaleDisplayCode(1),
        createdAtEpochMs = createdAtEpochMs,
        status = localSaleStatusToDisplay(status),
        totalAmount = totalAmount,
        totalCurrency = totalCurrency,
    )
}

private fun Sale.toRemoteListItem(
    parseCreatedAt: (String?) -> Long,
    codes: Map<String, String>,
): SalesListItem =
    SalesListItem(
        navigationId = id,
        shortId = displayCode ?: codes[id] ?: formatSaleDisplayCode(1),
        createdAtEpochMs = parseCreatedAt(createdAt),
        status = remoteSaleStatusToDisplay(status),
        totalAmount = totalAmount,
        totalCurrency = totalCurrency,
    )

internal expect fun parseIso8601EpochMs(iso: String?): Long
