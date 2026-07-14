package com.fullsales.seller.shared.db.repository

import com.fullsales.seller.shared.db.dao.CatalogDao
import com.fullsales.seller.shared.db.dao.RegistrationDao
import com.fullsales.seller.shared.db.entity.SyncMetadataEntity
import com.fullsales.seller.shared.db.mapper.toEntity
import com.fullsales.seller.shared.db.mapper.toModel
import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.LocalRegistration
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.model.generateUuidV7
import com.fullsales.seller.shared.repository.RegistrationRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class RoomRegistrationRepository(
    private val dao: RegistrationDao,
    private val catalogDao: CatalogDao,
    private val json: Json = Json { ignoreUnknownKeys = true },
) : RegistrationRepository {
    override fun observeRegistrations(): Flow<List<LocalRegistration>> =
        dao.observeAll().map { rows -> rows.map { it.toModel() } }

    override suspend fun getRegistration(localId: String): LocalRegistration? =
        dao.getByLocalId(localId)?.toModel()

    override suspend fun getByRemoteId(remoteId: String): LocalRegistration? =
        dao.getByRemoteId(remoteId)?.toModel()

    override suspend fun createPending(
        request: SubmitRegistrationRequest,
        idempotencyKey: String,
    ): LocalRegistration {
        val now = currentEpochMs()
        val localId = generateUuidV7()
        val row = LocalRegistration(
            localId = localId,
            cnpj = request.cnpj,
            legalName = request.legalName,
            tradeName = request.tradeName.orEmpty(),
            active = false,
            registrationStatus = "PendingReview",
            registrationMode = request.registrationMode,
            contactPhone = request.contact.phone,
            contactEmail = request.contact.email,
            deliveryAddressJson = json.encodeToString(request.deliveryAddress),
            syncStatus = LocalRegistrationSyncStatus.PendingSync,
            createdAtEpochMs = now,
            updatedAtEpochMs = now,
            idempotencyKey = idempotencyKey,
        )
        dao.upsert(row.toEntity())
        return row
    }

    override suspend fun upsertFromRemote(remote: List<CommerceRegistration>) {
        remote.forEach { upsertSyncedRemote(it) }
    }

    override suspend fun upsertSyncedRemote(remote: CommerceRegistration) {
        val existing = getByRemoteId(remote.id) ?: getRegistration(remote.id)
        val now = currentEpochMs()
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
        dao.upsert(row.toEntity())
    }

    override suspend fun setRemoteSynced(
        localId: String,
        remoteId: String,
        registrationStatus: String,
        active: Boolean,
    ) {
        dao.setRemoteSynced(
            localId = localId,
            remoteId = remoteId,
            syncStatus = LocalRegistrationSyncStatus.Synced.name,
            registrationStatus = registrationStatus,
            active = active,
            updatedAt = currentEpochMs(),
        )
    }

    override suspend fun markSyncFailed(localId: String, reason: String) {
        dao.markSyncFailed(
            localId = localId,
            syncStatus = LocalRegistrationSyncStatus.SyncFailed.name,
            reason = reason,
            updatedAt = currentEpochMs(),
        )
    }

    override suspend fun getLastRegistrationsSyncEpochMs(): Long? =
        catalogDao.getMetadata(KEY_LAST_REG_SYNC)?.toLongOrNull()

    override suspend fun setLastRegistrationsSyncEpochMs(epochMs: Long) {
        catalogDao.upsertMetadata(SyncMetadataEntity(KEY_LAST_REG_SYNC, epochMs.toString()))
    }

    private companion object {
        const val KEY_LAST_REG_SYNC = "lastRegistrationsSync"
    }
}
