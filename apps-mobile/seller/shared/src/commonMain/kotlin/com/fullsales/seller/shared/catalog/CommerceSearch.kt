package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.shared.search.smartSearchRank
import com.fullsales.seller.shared.search.textMatchesSmartSearch

fun commerceMatchesSearch(commerce: Commerce, query: String): Boolean {
    if (query.trim().isEmpty()) return true
    return textMatchesSmartSearch(commerce.legalName, query) ||
        textMatchesSmartSearch(commerce.tradeName.orEmpty(), query) ||
        textMatchesSmartSearch(commerce.displayName(), query)
}

fun filterCommercesBySearch(commerces: List<Commerce>, query: String): List<Commerce> =
    commerces
        .asSequence()
        .filter { commerceMatchesSearch(it, query) }
        .sortedWith(
            compareBy<Commerce> { smartSearchRank(it.displayName(), query) }
                .thenBy { it.displayName().lowercase() },
        )
        .toList()
