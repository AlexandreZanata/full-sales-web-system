package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSale

/**
 * Visual-only 8-character alphanumeric code for sellers (not a UUID / navigation id).
 * Sequence is assigned by [LocalSale.createdAtEpochMs] ascending (oldest = 00000001).
 */
fun formatSaleDisplayCode(sequence: Int): String {
    require(sequence >= 1) { "sequence must be >= 1" }
    return sequence.toString(radix = 36).uppercase().padStart(8, '0')
}

/** Maps localId and remoteId (when present) to the same display code. */
fun saleDisplayCodes(local: List<LocalSale>): Map<String, String> {
    val codes = LinkedHashMap<String, String>()
    local
        .sortedWith(compareBy({ it.createdAtEpochMs }, { it.localId }))
        .forEachIndexed { index, sale ->
            val code = formatSaleDisplayCode(index + 1)
            codes[sale.localId] = code
            sale.remoteId?.let { codes[it] = code }
        }
    return codes
}

fun saleDisplayCodeFor(local: List<LocalSale>, navigationId: String): String =
    saleDisplayCodes(local)[navigationId] ?: formatSaleDisplayCode(1)
