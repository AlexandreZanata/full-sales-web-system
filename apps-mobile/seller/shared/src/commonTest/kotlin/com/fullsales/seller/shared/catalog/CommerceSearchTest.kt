package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Commerce
import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Contract: commerce search uses smart matching (accents, tokens, small typos).
 */
class CommerceSearchTest {
    private val sorriso = Commerce(
        id = "c1",
        legalName = "PREFEITURA MUNICIPAL DE SORRISO",
        tradeName = "SORRISO GABINETE DO PREFEITO",
        active = true,
    )

    private val saoPaulo = Commerce(
        id = "c2",
        legalName = "MERCADO SAO PAULO LTDA",
        tradeName = "Mercado São Paulo",
        active = true,
    )

    @Test
    fun given_second_word_when_search_then_matches_trade_name() {
        assertTrue(commerceMatchesSearch(sorriso, "gabinete"))
        assertTrue(commerceMatchesSearch(sorriso, "prefeito"))
    }

    @Test
    fun given_missing_accent_when_search_then_matches() {
        assertTrue(commerceMatchesSearch(saoPaulo, "sao paulo"))
        assertTrue(commerceMatchesSearch(saoPaulo, "são"))
    }

    @Test
    fun given_small_typo_when_search_then_matches() {
        assertTrue(commerceMatchesSearch(sorriso, "soriso"))
    }

    @Test
    fun given_unrelated_when_search_then_no_match() {
        assertFalse(commerceMatchesSearch(sorriso, "beta market"))
    }

    @Test
    fun empty_query_matches_all() {
        assertTrue(commerceMatchesSearch(sorriso, ""))
    }
}
