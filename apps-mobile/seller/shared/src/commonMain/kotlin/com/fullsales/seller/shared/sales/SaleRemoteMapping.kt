package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleOrigin

/**
 * Maps API Sale → LocalStore row (OD-16-3: mirrors use localId = remote id).
 */
fun Sale.toMirroredLocalSale(
    parseCreatedAt: (String?) -> Long = ::parseIso8601EpochMs,
    existingLocalId: String? = null,
    existingOrigin: SaleOrigin? = null,
    existingIdempotencyKey: String? = null,
    existingDisplayCode: String? = null,
): LocalSale = LocalSale(
    localId = existingLocalId ?: id,
    remoteId = id,
    idempotencyKey = existingIdempotencyKey ?: id,
    commerceId = commerceId,
    paymentMethod = paymentMethod,
    status = remoteStatusToLocal(status),
    totalAmount = totalAmount,
    totalCurrency = totalCurrency,
    items = items,
    createdAtEpochMs = parseCreatedAt(createdAt),
    driverId = driverId,
    origin = existingOrigin ?: SaleOrigin.RemoteMirror,
    displayCode = displayCode ?: existingDisplayCode,
)

fun remoteStatusToLocal(status: String): LocalSaleStatus = when (status) {
    "Confirmed" -> LocalSaleStatus.Confirmed
    "Cancelled" -> LocalSaleStatus.Cancelled
    else -> LocalSaleStatus.Synced // server Pending (or unknown) → display Pending
}
