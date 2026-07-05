package com.fullsales.seller.shared.model

import kotlin.random.Random

/** Time-sortable UUID v7 (RFC 9562) for local sale and idempotency keys. */
fun generateUuidV7(clockMs: Long = currentEpochMs()): String {
    val bytes = ByteArray(16)
    writeTimestamp(clockMs, bytes)
    Random.nextBytes(bytes, 6, 10)
    bytes[6] = ((bytes[6].toInt() and 0x0F) or 0x70).toByte()
    bytes[8] = ((bytes[8].toInt() and 0x3F) or 0x80).toByte()
    return formatUuid(bytes)
}

/** Wall-clock epoch ms for sync timestamps and UUID v7. */
expect fun currentEpochMs(): Long

private fun writeTimestamp(clockMs: Long, bytes: ByteArray) {
    var value = clockMs
    for (index in 5 downTo 0) {
        bytes[index] = (value and 0xFF).toByte()
        value = value ushr 8
    }
}

private fun formatUuid(bytes: ByteArray): String = buildString(36) {
    bytes.forEachIndexed { index, byte ->
        if (index == 4 || index == 6 || index == 8 || index == 10) append('-')
        append(hex[(byte.toInt() shr 4) and 0xF])
        append(hex[byte.toInt() and 0xF])
    }
}

private val hex = charArrayOf('0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f')
