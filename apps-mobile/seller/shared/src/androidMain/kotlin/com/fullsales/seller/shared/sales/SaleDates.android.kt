package com.fullsales.seller.shared.sales

import java.time.Instant

internal actual fun parseIso8601EpochMs(iso: String?): Long =
    iso?.let { runCatching { Instant.parse(it).toEpochMilli() }.getOrNull() } ?: 0L
