package com.fullsales.seller.shared.model

import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

/** Formats minor-unit amount (centavos) as BRL currency. */
fun formatMoneyMinorUnits(amountMinor: Long, currency: String = "BRL"): String {
    require(currency == "BRL") { "Only BRL formatting is supported" }
    val sign = if (amountMinor < 0) "-" else ""
    val abs = kotlin.math.abs(amountMinor)
    val reais = abs / 100
    val centavos = abs % 100
    return buildString {
        if (sign.isNotEmpty()) append(sign)
        append("R$ ")
        append(groupThousands(reais))
        append(',')
        append(centavos.toString().padStart(2, '0'))
    }
}

fun formatProductPrice(priceAmount: Double, currency: String): String =
    formatMoneyMinorUnits(priceAmount.toLong(), currency)

/** dd/MM/yyyy HH:mm in the device time zone. */
fun formatSalesListDateTime(epochMs: Long): String {
    val dt = Instant.fromEpochMilliseconds(epochMs).toLocalDateTime(TimeZone.currentSystemDefault())
    return buildString {
        append(dt.dayOfMonth.toString().padStart(2, '0'))
        append('/')
        append(dt.monthNumber.toString().padStart(2, '0'))
        append('/')
        append(dt.year)
        append(' ')
        append(dt.hour.toString().padStart(2, '0'))
        append(':')
        append(dt.minute.toString().padStart(2, '0'))
    }
}

fun stockBadgeLabel(available: Int?): String = when {
    available == null -> "Stock unknown"
    available <= 0 -> "Unavailable"
    else -> "Available: $available"
}

fun isStockUnavailable(available: Int?): Boolean =
    available != null && available <= 0

private fun groupThousands(value: Long): String {
    val raw = value.toString()
    if (raw.length <= 3) return raw
    val builder = StringBuilder()
    val leading = raw.length % 3
    if (leading > 0) {
        builder.append(raw.substring(0, leading))
        if (raw.length > leading) builder.append('.')
    }
    raw.substring(if (leading > 0) leading else 0).chunked(3).forEachIndexed { index, chunk ->
        if (index > 0) builder.append('.')
        builder.append(chunk)
    }
    return builder.toString()
}
