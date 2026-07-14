package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest

/**
 * T-16-08 — registration pull caches LocalStore; offline observe returns cache.
 */
class RegistrationRepositoryTest {
    @Test
    fun given_remoteRegistrationFixture_when_pull_then_localRowsCached() = runTest {
        val registrations = FakeRegistrationRepository()
        val client = FakeCatalogPullClient().apply {
            this.registrations = listOf(remoteReg("reg-1", status = "PendingReview"))
        }
        PullRegistrationsSync(registrations, client).pullRegistrations(nowEpochMs = 42_000L)

        val stored = registrations.getByRemoteId("reg-1")
        assertNotNull(stored)
        assertEquals("reg-1", stored.localId)
        assertEquals(LocalRegistrationSyncStatus.Synced, stored.syncStatus)
        assertEquals("PendingReview", stored.registrationStatus)
        assertEquals(42_000L, registrations.getLastRegistrationsSyncEpochMs())
        assertEquals(1, registrations.observeRegistrations().first().size)
    }

    @Test
    fun given_cachedRegistrations_when_offlineObserve_then_returnsCached() = runTest {
        val registrations = FakeRegistrationRepository()
        registrations.upsertSyncedRemote(remoteReg("cached-1", status = "Active"))
        val observed = registrations.observeRegistrations().first()
        assertEquals(1, observed.size)
        assertEquals("cached-1", observed.single().remoteId)
        assertEquals(LocalRegistrationSyncStatus.Synced, observed.single().syncStatus)
    }

    @Test
    fun given_duplicateRemoteId_when_pullTwice_then_singleRow() = runTest {
        val registrations = FakeRegistrationRepository()
        val client = FakeCatalogPullClient().apply {
            this.registrations = listOf(remoteReg("dup", status = "PendingReview"))
        }
        val sync = PullRegistrationsSync(registrations, client)
        sync.pullRegistrations(nowEpochMs = 1L)
        client.registrations = listOf(remoteReg("dup", status = "Active"))
        sync.pullRegistrations(nowEpochMs = 2L)

        assertEquals(1, registrations.observeRegistrations().first().size)
        assertEquals("Active", registrations.getByRemoteId("dup")!!.registrationStatus)
        assertEquals(2L, registrations.getLastRegistrationsSyncEpochMs())
    }

    private fun remoteReg(id: String, status: String) = CommerceRegistration(
        id = id,
        cnpj = "11222333000181",
        legalName = "Acme",
        tradeName = "Acme Store",
        active = status == "Active",
        registrationStatus = status,
        registrationMode = "manual",
    )
}
