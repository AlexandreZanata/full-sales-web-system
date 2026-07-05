package com.fullsales.seller.shared.auth

private const val BASE64_TABLE = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

internal fun decodeBase64Url(value: String): ByteArray {
    val normalized = value.replace('-', '+').replace('_', '/')
    val pad = (4 - normalized.length % 4) % 4
    return decodeBase64(normalized + "=".repeat(pad))
}

internal fun encodeBase64Url(value: ByteArray): String =
    encodeBase64(value).trimEnd('=').replace('+', '-').replace('/', '_')

private fun decodeBase64(input: String): ByteArray {
    val result = mutableListOf<Byte>()
    var buffer = 0
    var bits = 0
    for (char in input) {
        if (char == '=') break
        val index = BASE64_TABLE.indexOf(char)
        if (index < 0) continue
        buffer = (buffer shl 6) or index
        bits += 6
        if (bits >= 8) {
            bits -= 8
            result.add(((buffer shr bits) and 0xFF).toByte())
        }
    }
    return result.toByteArray()
}

private fun encodeBase64(input: ByteArray): String {
    if (input.isEmpty()) return ""
    val out = StringBuilder()
    var index = 0
    while (index < input.size) {
        val b0 = input[index++].toInt() and 0xFF
        val hasB1 = index < input.size
        val b1 = if (hasB1) input[index++].toInt() and 0xFF else 0
        val hasB2 = index < input.size
        val b2 = if (hasB2) input[index++].toInt() and 0xFF else 0
        out.append(BASE64_TABLE[b0 shr 2])
        out.append(BASE64_TABLE[((b0 and 0x03) shl 4) or (b1 shr 4)])
        out.append(if (hasB1) BASE64_TABLE[((b1 and 0x0F) shl 2) or (b2 shr 6)] else '=')
        out.append(if (hasB2) BASE64_TABLE[b2 and 0x3F] else '=')
    }
    return out.toString()
}
