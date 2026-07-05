package com.fullsales.seller.shared.api

import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import kotlin.test.Test
import kotlin.test.assertEquals

class CreateSaleRequestContractTest {
    @Test
    fun createSaleRequestJson_matchesApiContractExample() {
        val request = CreateSaleRequest(
            commerceId = "0192a1b2-c3d4-7890-abcd-ef1234567890",
            items = listOf(
                CreateSaleItem(
                    productId = "0192a1b2-c3d4-7890-abcd-ef1234567891",
                    quantity = 2,
                ),
            ),
            paymentMethod = "cash",
        )
        val encoded = testJson().encodeToString(CreateSaleRequest.serializer(), request)
        val expected = """
            {"commerceId":"0192a1b2-c3d4-7890-abcd-ef1234567890","items":[{"productId":"0192a1b2-c3d4-7890-abcd-ef1234567891","quantity":2}],"paymentMethod":"cash"}
        """.trimIndent()
        assertEquals(expected, encoded)
    }
}
