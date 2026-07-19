package com.fullsales.seller.shared.search

/**
 * Accent-insensitive, typo-tolerant text match for seller catalog search.
 *
 * Contract (user-facing):
 * - GIVEN missing accents WHEN search THEN still match (sao → São)
 * - GIVEN second word only WHEN search THEN match (cola → Coca-Cola)
 * - GIVEN one-letter typo WHEN search THEN match (coco → Coca)
 */

fun normalizeSearchText(raw: String): String {
    val folded = buildString(raw.length) {
        for (ch in raw.trim().lowercase()) {
            when {
                ch.isLetterOrDigit() -> append(foldLatinChar(ch))
                ch.isWhitespace() || ch == '-' || ch == '_' || ch == '/' -> append(' ')
            }
        }
    }
    return folded.split(' ').filter { it.isNotEmpty() }.joinToString(" ")
}

fun textMatchesSmartSearch(haystack: String, query: String): Boolean {
    val normalizedQuery = normalizeSearchText(query)
    if (normalizedQuery.isEmpty()) return true
    val normalizedHaystack = normalizeSearchText(haystack)
    if (normalizedHaystack.isEmpty()) return false
    if (normalizedHaystack.contains(normalizedQuery)) return true
    val compactHay = normalizedHaystack.replace(" ", "")
    val compactQuery = normalizedQuery.replace(" ", "")
    if (compactHay.contains(compactQuery)) return true
    val queryTokens = normalizedQuery.split(' ')
    val hayTokens = normalizedHaystack.split(' ')
    return queryTokens.all { queryToken ->
        hayTokens.any { hayToken -> tokenMatches(hayToken, queryToken) }
    }
}

fun smartSearchRank(haystack: String, query: String): Int {
    val q = normalizeSearchText(query)
    if (q.isEmpty()) return 0
    val h = normalizeSearchText(haystack)
    if (h.contains(q) || h.replace(" ", "").contains(q.replace(" ", ""))) return 0
    val qTokens = q.split(' ')
    val hTokens = h.split(' ')
    val allExactOrPrefix = qTokens.all { qt ->
        hTokens.any { ht -> ht == qt || ht.startsWith(qt) || qt.startsWith(ht) }
    }
    return if (allExactOrPrefix) 1 else 2
}

internal fun tokenMatches(hayToken: String, queryToken: String): Boolean {
    if (hayToken == queryToken) return true
    if (hayToken.startsWith(queryToken) || queryToken.startsWith(hayToken)) return true
    if (queryToken.length >= 3 && hayToken.contains(queryToken)) return true
    if (hayToken.length >= 3 && queryToken.contains(hayToken)) return true
    val maxDistance = maxEditDistance(queryToken.length)
    if (maxDistance == 0) return false
    return levenshtein(hayToken, queryToken) <= maxDistance
}

internal fun maxEditDistance(tokenLength: Int): Int = when {
    tokenLength <= 2 -> 0
    tokenLength <= 5 -> 1
    else -> 2
}

internal fun levenshtein(a: String, b: String): Int {
    if (a == b) return 0
    if (a.isEmpty()) return b.length
    if (b.isEmpty()) return a.length
    val previous = IntArray(b.length + 1) { it }
    val current = IntArray(b.length + 1)
    for (i in 1..a.length) {
        current[0] = i
        for (j in 1..b.length) {
            val cost = if (a[i - 1] == b[j - 1]) 0 else 1
            current[j] = minOf(
                current[j - 1] + 1,
                previous[j] + 1,
                previous[j - 1] + cost,
            )
        }
        for (j in previous.indices) previous[j] = current[j]
    }
    return previous[b.length]
}

private fun foldLatinChar(ch: Char): Char = when (ch) {
    'á', 'à', 'â', 'ã', 'ä', 'å', 'ā', 'ă', 'ą' -> 'a'
    'é', 'è', 'ê', 'ë', 'ē', 'ė', 'ę' -> 'e'
    'í', 'ì', 'î', 'ï', 'ī', 'į' -> 'i'
    'ó', 'ò', 'ô', 'õ', 'ö', 'ō', 'ő' -> 'o'
    'ú', 'ù', 'û', 'ü', 'ū', 'ů', 'ű' -> 'u'
    'ý', 'ÿ' -> 'y'
    'ç', 'ć', 'č' -> 'c'
    'ñ', 'ń' -> 'n'
    'ś', 'š', 'ş' -> 's'
    'ź', 'ż', 'ž' -> 'z'
    'ř' -> 'r'
    'ď' -> 'd'
    'ť' -> 't'
    'ł' -> 'l'
    else -> ch
}
