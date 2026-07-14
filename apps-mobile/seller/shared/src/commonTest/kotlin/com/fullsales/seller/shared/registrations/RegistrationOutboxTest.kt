package com.fullsales.seller.shared.registrations

import com.fullsales.seller.shared.api.RecordedMockEngine
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.api.testClient
import com.fullsales.seller.shared.model.DeliveryAddressRequest
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import com.fullsales.seller.shared.model.RegistrationContact
import com.fullsales.seller.shared.model.RegistrationMode
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.sync.FakeOutboxRepository
import com.fullsales.seller.shared.sync.FakeRegistrationRepository
import com.fullsales.seller.shared.sync.OfflineRegistrationWriter
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlinx.coroutines.test.runTest

/**
 * T-16-07 — offline (and transport-fallback) registration submit → PendingSync + outbox POST.
 */
class RegistrationOutboxTest {
    private val registrations = FakeRegistrationRepository()
    private val outbox = FakeOutboxRepository()
    private val writer = OfflineRegistrationWriter(registrations, outbox)
    private val unusedApi = SellerApiClient(
        HttpClient(MockEngine { error("online path not used") }),
        baseUrl = "http://test/v1",
    )

    @Test
    fun given_offlineSubmit_when_submit_then_pendingSyncAndOutboxPost() = runTest {
        val submitter = CreateRegistrationSubmitter(unusedApi, writer, registrations) {
            "idem-reg-1"
        }
        val result = submitter.submit(sampleRequest(), online = false)
        assertIs<CreateRegistrationSubmitResult.Success>(result)
        assertEquals(false, result.isRemote)
        val stored = registrations.getRegistration(result.navigationId)
        assertEquals(LocalRegistrationSyncStatus.PendingSync, stored?.syncStatus)
        val pending = outbox.all.filter { !it.completed }
        assertEquals(1, pending.size)
        assertEquals("POST", pending.single().method)
        assertEquals("/commerces/registrations", pending.single().path)
        assertEquals(SyncEntityType.Registration, pending.single().entityType)
        assertEquals("idem-reg-1", pending.single().idempotencyKey)
    }

    @Test
    fun given_onlineTransportFailure_when_submit_then_fallsBackToOutbox() = runTest {
        val failingApi = SellerApiClient(
            HttpClient(MockEngine { error("connection timeout") }),
            baseUrl = "http://test/v1",
        )
        val submitter = CreateRegistrationSubmitter(failingApi, writer, registrations) {
            "idem-reg-fallback"
        }
        val result = submitter.submit(sampleRequest(), online = true)
        assertIs<CreateRegistrationSubmitResult.Success>(result)
        assertEquals(false, result.isRemote)
        assertEquals(1, outbox.all.count { !it.completed })
        assertEquals(
            LocalRegistrationSyncStatus.PendingSync,
            registrations.getRegistration(result.navigationId)?.syncStatus,
        )
    }

    @Test
    fun given_onlineCreateSuccess_when_submit_then_localStoreHasSyncedRow() = runTest {
        val recorder = RecordedMockEngine {
            HttpStatusCode.Created to """
                {"id":"remote-reg-9","cnpj":"11222333000181","legalName":"Acme",
                "tradeName":"Acme","active":false,"registrationStatus":"PendingReview"}
            """.trimIndent()
        }
        val submitter = CreateRegistrationSubmitter(
            testClient(engine = recorder.engine()),
            writer,
            registrations,
        ) { "idem-online" }
        val result = submitter.submit(sampleRequest(), online = true)
        assertIs<CreateRegistrationSubmitResult.Success>(result)
        assertEquals(true, result.isRemote)
        assertEquals("remote-reg-9", result.navigationId)
        val stored = registrations.getByRemoteId("remote-reg-9")
        assertEquals(LocalRegistrationSyncStatus.Synced, stored?.syncStatus)
        assertEquals(0, outbox.all.count { !it.completed })
        assertEquals("idem-online", recorder.requests.last().headers["Idempotency-Key"])
    }

    private fun sampleRequest() = SubmitRegistrationRequest(
        cnpj = "11222333000181",
        legalName = "Acme",
        tradeName = "Acme Store",
        contact = RegistrationContact(phone = "11999999999"),
        deliveryAddress = DeliveryAddressRequest(
            street = "Rua A",
            number = "1",
            city = "SP",
            state = "SP",
            postalCode = "01000",
        ),
        registrationMode = RegistrationMode.MANUAL,
    )
}
