package com.fullsales.seller.shared.sales

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class CreateSaleDraftTest {
    @Test
    fun encodeDecode_roundTripsDraftFields() {
        val draft = CreateSaleDraft(
            commerceId = "commerce-1",
            paymentMethod = "cash",
            lines = listOf(CreateSaleLineInput(productId = "p1", quantityText = "3")),
        )
        val decoded = CreateSaleDraftCodec.decode(CreateSaleDraftCodec.encode(draft))
        assertNotNull(decoded)
        assertEquals(draft, decoded)
    }

    @Test
    fun isEffectivelyEmpty_whenDefaultFields_thenTrue() {
        assertTrue(CreateSaleDraft().isEffectivelyEmpty())
    }

    @Test
    fun isEffectivelyEmpty_whenCommerceSelected_thenFalse() {
        assertFalse(CreateSaleDraft(commerceId = "c1").isEffectivelyEmpty())
    }

    @Test
    fun createSaleDraftFrom_stripsProductSearchQuery() {
        val draft = createSaleDraftFrom(
            commerceId = "c1",
            paymentMethod = "pix",
            lines = listOf(CreateSaleLineInput(productId = "p1", productSearchQuery = "cola")),
        )
        assertEquals("", draft.lines.first().productSearchQuery)
    }
}
