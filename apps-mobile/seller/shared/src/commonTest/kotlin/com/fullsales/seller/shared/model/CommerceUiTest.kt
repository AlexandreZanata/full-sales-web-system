package com.fullsales.seller.shared.model

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class CommerceUiTest {
    @Test
    fun toUiModel_mapsAddressDtoToDisplayFields() {
        val address = CommerceAddress(
            id = "a1",
            type = "Delivery",
            street = "Rua A",
            number = "100",
            city = "São Paulo",
            state = "SP",
            postalCode = "01000-000",
            isPrimary = true,
        )

        val ui = address.toUiModel()

        assertEquals("a1", ui.id)
        assertEquals("Delivery", ui.typeLabel)
        assertEquals("Rua A, 100", ui.streetLine)
        assertEquals("São Paulo — SP 01000-000", ui.cityLine)
        assertTrue(ui.isPrimary)
    }

    @Test
    fun toUiModel_handlesBillingType() {
        val ui = CommerceAddress(
            id = "b1",
            type = "Billing",
            street = "Av B",
            number = "50",
            city = "Curitiba",
            state = "PR",
            postalCode = "80000-000",
        ).toUiModel()

        assertEquals("Billing", ui.typeLabel)
        assertFalse(ui.isPrimary)
    }

    @Test
    fun maskCnpj_hidesMiddleDigits() {
        assertEquals("12.***.***/****-90", maskCnpj("12.345.678/0001-90"))
    }

    @Test
    fun displayName_prefersTradeName() {
        val commerce = Commerce(
            id = "c1",
            legalName = "Legal Corp LTDA",
            tradeName = "Trade Shop",
            active = true,
        )
        assertEquals("Trade Shop", commerce.displayName())
    }
}
