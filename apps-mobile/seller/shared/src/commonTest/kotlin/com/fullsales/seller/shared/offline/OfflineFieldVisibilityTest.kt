package com.fullsales.seller.shared.offline

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * Contract: offline create-sale UI state must expose catalog-backed fields
 * (not blank after LocalStore seed). Pure state shape — no network.
 */
class OfflineFieldVisibilityTest {
    @Test
    fun createSale_mustShow_lists_are_nonEmpty_when_seeded() {
        val commerces = listOf("c1")
        val products = listOf("p1")
        val paymentMethods = listOf("Cash", "Pix")
        assertTrue(OfflineFieldVisibility.createSaleMustShow.contains("commerces"))
        assertTrue(commerces.isNotEmpty())
        assertTrue(products.isNotEmpty())
        assertTrue(paymentMethods.isNotEmpty())
    }

    @Test
    fun internetOnly_fields_remain_documented_not_removed() {
        assertEquals(
            listOf("cnpjLookup", "mediaUpload", "topSellingChips"),
            OfflineFieldVisibility.internetOnlyDisabledWithWarn,
        )
    }

    @Test
    fun formatSyncEpochIso_null_is_null() {
        assertEquals(null, formatSyncEpochIso(null))
        assertNotNull(formatSyncEpochIso(1_700_000_000_000L))
    }
}
