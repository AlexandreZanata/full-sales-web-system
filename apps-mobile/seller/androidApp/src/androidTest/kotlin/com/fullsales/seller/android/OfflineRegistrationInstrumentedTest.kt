package com.fullsales.seller.android

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.fullsales.seller.shared.model.DeliveryAddressRequest
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import com.fullsales.seller.shared.model.RegistrationContact
import com.fullsales.seller.shared.model.RegistrationMode
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.model.toCommerceRegistration
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

/**
 * T-16-33 — Airplane path: offline submit → PendingSync visible in LocalStore list.
 */
@RunWith(AndroidJUnit4::class)
class OfflineRegistrationInstrumentedTest {
    private lateinit var container: AppContainer

    @Before
    fun setUp() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        context.deleteDatabase("seller.db")
        container = AppContainer(context)
    }

    @Test
    fun given_offlineSubmit_when_observe_then_listShowsPending() = runBlocking {
        val local = container.offlineRegistrationWriter.enqueue(
            SubmitRegistrationRequest(
                cnpj = "11222333000181",
                legalName = "Acme Off",
                tradeName = "Acme Off",
                contact = RegistrationContact(),
                deliveryAddress = DeliveryAddressRequest(
                    street = "Rua B",
                    number = "2",
                    city = "RJ",
                    state = "RJ",
                    postalCode = "20000",
                ),
                registrationMode = RegistrationMode.MANUAL,
            ),
            idempotencyKey = "idem-air-reg",
        )
        val rows = container.registrationRepository.observeRegistrations().first()
        assertEquals(1, rows.size)
        assertEquals(local.localId, rows.single().localId)
        assertEquals(LocalRegistrationSyncStatus.PendingSync, rows.single().syncStatus)
        assertEquals("PendingSync", rows.single().toCommerceRegistration().registrationStatus)
        val pending = container.outboxRepository.listPendingFifo()
        assertTrue(pending.any { it.path == "/commerces/registrations" })
        assertEquals(SyncEntityType.Registration, pending.single().entityType)
    }
}
