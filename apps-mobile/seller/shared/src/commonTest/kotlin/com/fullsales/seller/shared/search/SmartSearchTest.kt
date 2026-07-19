package com.fullsales.seller.shared.search

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Contract: seller smart search — accents, second-word tokens, small typos.
 */
class SmartSearchTest {
    @Test
    fun given_missing_accent_when_search_then_matches() {
        assertTrue(textMatchesSmartSearch("São Paulo Mercado", "sao paulo"))
        assertTrue(textMatchesSmartSearch("Café Especial", "cafe"))
    }

    @Test
    fun given_second_word_only_when_search_then_matches_product() {
        assertTrue(textMatchesSmartSearch("Coca-Cola 2L", "cola"))
        assertTrue(textMatchesSmartSearch("Coca Cola 2L", "cola"))
    }

    @Test
    fun given_one_letter_typo_when_search_then_matches_coca() {
        assertTrue(textMatchesSmartSearch("Coca-Cola 2L", "coco"))
        assertTrue(textMatchesSmartSearch("Coca-Cola 2L", "coca"))
    }

    @Test
    fun given_unrelated_term_when_search_then_no_match() {
        assertFalse(textMatchesSmartSearch("Coca-Cola 2L", "guarana"))
    }

    @Test
    fun given_hyphenated_name_when_compact_query_then_matches() {
        assertTrue(textMatchesSmartSearch("Coca-Cola", "cocacola"))
    }

    @Test
    fun given_short_token_when_typo_then_does_not_overmatch() {
        assertFalse(textMatchesSmartSearch("Beta Market", "zz"))
    }

    @Test
    fun normalize_folds_accents_and_separators() {
        assertEquals("sao paulo", normalizeSearchText("  São-Paulo  "))
    }

    @Test
    fun levenshtein_distance_coco_coca_is_one() {
        assertEquals(1, levenshtein("coco", "coca"))
    }
}
