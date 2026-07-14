package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.LocalRegistration
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.RegistrationRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class FakeRegistrationRepository(
    private val json: Json = Json { ignoreUnknownKeys = true },
) : RegistrationRepository {
    private val mutex = Mutex()
    private val rows = linkedMapOf<String, LocalRegistration>()
    private val flow = MutableStateFlow<List<LocalRegistration>>(emptyList())
    private var lastSync: Long? = null

    override fun observeRegistrations(): Flow<List<LocalRegistration>> = flow.asStateFlow()

    override suspend fun getRegistration(localId: String): LocalRegistration? =
        mutex.withLock { rows[localId] }

    override suspend fun getByRemoteId(remoteId: String): LocalRegistration? = mutex.withLock {
        rows.values.firstOrNull { it.remoteId == remoteId }
    }

    override suspend fun createPending(
        request: SubmitRegistrationRequest,
        idempotencyKey: String,
    ): LocalRegistration = mutex.withLock {
        val now = runCatching { currentEpochMs() }.getOrDefault(1L)
        val localId = "local-reg-${rows.size + 1}"
        val row = LocalRegistration(
            localId = localId,
            cnpj = request.cnpj,
            legalName = request.legalName,
            tradeName = request.tradeName.orEmpty(),
            registrationMode = request.registrationMode,
            contactPhone = request.contact.phone,
            contactEmail = request.contact.email,
            deliveryAddressJson = json.encodeToString(request.deliveryAddress),
            syncStatus = LocalRegistrationSyncStatus.PendingSync,
            createdAtEpochMs = now,
            updatedAtEpochMs = now,
            idempotencyKey = idempotencyKey,
        )
        rows[localId] = row
        flow.value = rows.values.toList()
        row
    }

    override suspend fun upsertFromRemote(remote: List<CommerceRegistration>) {
        remote.forEach { upsertSyncedRemote(it) }
    }

    override suspend fun upsertSyncedRemote(remote: CommerceRegistration) {
        mutex.withLock {
            val existing = rows.values.firstOrNull { it.remoteId == remote.id } ?: rows[remote.id]
            val now = runCatching { currentEpochMs() }.getOrDefault(1L)
            val row = LocalRegistration(
                localId = existing?.localId ?: remote.id,
                remoteId = remote.id,
                cnpj = remote.cnpj,
                legalName = remote.legalName,
                tradeName = remote.tradeName,
                active = remote.active,
                registrationStatus = remote.registrationStatus,
                rejectionReason = remote.rejectionReason,
                registrationMode = remote.registrationMode,
                contactPhone = existing?.contactPhone,
                contactEmail = existing?.contactEmail,
                deliveryAddressJson = existing?.deliveryAddressJson ?: "{}",
                syncStatus = LocalRegistrationSyncStatus.Synced,
                createdAtEpochMs = existing?.createdAtEpochMs ?: now,
                updatedAtEpochMs = now,
                idempotencyKey = existing?.idempotencyKey ?: remote.id,
            )
            rows[row.localId] = row
            flow.value = rows.values.toList()
        }
    }

    override suspend fun setRemoteSynced(
        localId: String,
        remoteId: String,
        registrationStatus: String,
        active: Boolean,
    ) {
        mutex.withLock {
            rows[localId]?.let {
                rows[localId] = it.copy(
                    remoteId = remoteId,
                    registrationStatus = registrationStatus,
                    active = active,
                    syncStatus = LocalRegistrationSyncStatus.Synced,
                )
                flow.value = rows.values.toList()
            }
        }
    }

    override suspend fun markSyncFailed(localId: String, reason: String) {
        mutex.withLock {
            rows[localId]?.let {
                rows[localId] = it.copy(
                    syncStatus = LocalRegistrationSyncStatus.SyncFailed,
                    syncFailureReason = reason,
                )
                flow.value = rows.values.toList()
            }
        }
    }

    override suspend fun getLastRegistrationsSyncEpochMs(): Long? = lastSync

    override suspend fun setLastRegistrationsSyncEpochMs(epochMs: Long) {
        lastSync = epochMs
    }
}
