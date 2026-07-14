package com.fullsales.seller.shared.db

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.fullsales.seller.shared.db.repository.RoomRegistrationRepository
import com.fullsales.seller.shared.db.repository.RoomSyncOutboxRepository
import com.fullsales.seller.shared.model.DeliveryAddressRequest
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import com.fullsales.seller.shared.model.RegistrationContact
import com.fullsales.seller.shared.model.RegistrationMode
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.sync.OfflineRegistrationWriter
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

/**
 * T-16-22 — registration + outbox rows survive Room reopen.
 */
@RunWith(RobolectricTestRunner::class)
class RegistrationPersistenceTest {
    private lateinit var context: Context

    @Before
    fun setUp() {
        context = ApplicationProvider.getApplicationContext()
        context.deleteDatabase(DB_NAME)
    }

    @After
    fun tearDown() {
        context.deleteDatabase(DB_NAME)
    }

    @Test
    fun given_pendingRegistrationOutbox_when_reopenDb_then_rowsSurvive() = runTest {
        val db1 = SellerDatabase.build(context, DB_NAME)
        val regRepo1 = RoomRegistrationRepository(db1.registrationDao(), db1.catalogDao())
        val outbox1 = RoomSyncOutboxRepository(db1.syncOutboxDao())
        val writer = OfflineRegistrationWriter(regRepo1, outbox1)
        val local = writer.enqueue(
            SubmitRegistrationRequest(
                cnpj = "11222333000181",
                legalName = "Acme",
                tradeName = "Store",
                contact = RegistrationContact(email = "a@b.c"),
                deliveryAddress = DeliveryAddressRequest(
                    street = "Rua A",
                    number = "1",
                    city = "SP",
                    state = "SP",
                    postalCode = "01000",
                ),
                registrationMode = RegistrationMode.MANUAL,
            ),
            idempotencyKey = "idem-persist-reg",
        )
        regRepo1.setLastRegistrationsSyncEpochMs(77L)
        db1.close()

        val db2 = SellerDatabase.build(context, DB_NAME)
        val regRepo2 = RoomRegistrationRepository(db2.registrationDao(), db2.catalogDao())
        val outbox2 = RoomSyncOutboxRepository(db2.syncOutboxDao())
        val restored = regRepo2.getRegistration(local.localId)
        val pending = outbox2.listPendingFifo()
        val syncAt = regRepo2.getLastRegistrationsSyncEpochMs()
        val count = regRepo2.observeRegistrations().first().size
        db2.close()

        assertNotNull(restored)
        assertEquals(LocalRegistrationSyncStatus.PendingSync, restored!!.syncStatus)
        assertEquals("11222333000181", restored.cnpj)
        assertTrue(restored.deliveryAddressJson.contains("Rua A"))
        assertEquals(1, pending.size)
        assertEquals("/commerces/registrations", pending.single().path)
        assertEquals(SyncEntityType.Registration, pending.single().entityType)
        assertEquals(77L, syncAt)
        assertEquals(1, count)
    }

    private companion object {
        const val DB_NAME = "seller-reg-persist.db"
    }
}
