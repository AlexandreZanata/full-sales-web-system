package com.fullsales.seller.shared.api

import com.fullsales.seller.shared.model.RegistrationMode
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

class SellerApiRegistrationsTest {
    @Test
    fun lookupCnpj_sendsDigitsQuery() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/commerces/cnpj-lookup", request.url.encodedPath)
            assertTrue(request.url.encodedQuery.contains("cnpj=11222333000181"))
            HttpStatusCode.OK to """
                {"cnpj":"11222333000181","legalName":"Acme","tradeName":"Acme Store","address":{"street":"Rua A","number":"1","district":"Centro","city":"SP","state":"SP","postalCode":"01000"},"provider":"mock","fetchedAt":"2026-01-01T00:00:00Z"}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val result = client.lookupCnpj("11.222.333/0001-81")
        assertEquals("Acme", result.legalName)
    }

    @Test
    fun submitRegistration_postsPayload() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/commerces/registrations", request.url.encodedPath)
            assertEquals("Bearer seller-token", request.authHeader())
            HttpStatusCode.Created to """
                {"id":"r1","cnpj":"11222333000181","legalName":"Acme","tradeName":"Acme","active":false,"registrationStatus":"PendingReview"}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val result = client.submitRegistration(sampleSubmit())
        assertEquals("PendingReview", result.registrationStatus)
    }

    @Test
    fun listRegistrations_usesSellerScope() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/commerces/registrations", request.url.encodedPath)
            assertTrue(request.url.encodedQuery.contains("limit=20"))
            HttpStatusCode.OK to """{"data":[],"pagination":{"next_cursor":null,"has_more":false,"limit":20}}"""
        }
        val client = testClient(engine = recorder.engine())
        client.listRegistrations(limit = 20)
        assertEquals(1, recorder.requests.size)
    }

    @Test
    fun lookupCnpj_notFoundMapsApiError() = runTest {
        val recorder = RecordedMockEngine { _ ->
            HttpStatusCode.NotFound to """
                {"error":{"code":"CNPJ_NOT_FOUND","message":"Not found","correlationId":"00000000-0000-0000-0000-000000000001"}}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val error = assertFailsWith<ApiException> { client.lookupCnpj("11222333000181") }
        assertEquals("CNPJ_NOT_FOUND", error.detail.code)
    }

    private fun sampleSubmit() = com.fullsales.seller.shared.model.SubmitRegistrationRequest(
        cnpj = "11222333000181",
        legalName = "Acme",
        tradeName = "Acme Store",
        contact = com.fullsales.seller.shared.model.RegistrationContact(),
        deliveryAddress = com.fullsales.seller.shared.model.DeliveryAddressRequest(
            street = "Rua A",
            number = "1",
            city = "SP",
            state = "SP",
            postalCode = "01000",
        ),
        registrationMode = RegistrationMode.MANUAL,
    )
}
