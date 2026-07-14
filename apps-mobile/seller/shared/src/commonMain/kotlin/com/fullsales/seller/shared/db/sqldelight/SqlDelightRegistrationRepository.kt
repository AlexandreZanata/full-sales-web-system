package com.fullsales.seller.shared.db.sqldelight

import app.cash.sqldelight.coroutines.asFlow
import app.cash.sqldelight.coroutines.mapToList
import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.LocalRegistration
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.model.generateUuidV7
import com.fullsales.seller.shared.repository.RegistrationRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class SqlDelightRegistrationRepository(
    private val db: SellerLocalDatabase,
    private val json: Json = Json { ignoreUnknownKeys = true },
) : RegistrationRepository {
    private val q get() = db.registrationsQueries
    private val catalog get() = db.catalogQueries

    override fun observeRegistrations(): Flow<List<LocalRegistration>> =
        q.selectAllRegistrations().asFlow().mapToList(Dispatchers.Default)
            .map { rows -> rows.map { it.toModel() } }

    override suspend fun getRegistration(localId: String): LocalRegistration? =
        q.selectRegistrationByLocalId(localId).executeAsOneOrNull()?.toModel()

    override suspend fun getByRemoteId(remoteId: String): LocalRegistration? =
        q.selectRegistrationByRemoteId(remoteId).executeAsOneOrNull()?.toModel()

    override suspend fun createPending(
        request: SubmitRegistrationRequest,
        idempotencyKey: String,
    ): LocalRegistration {
        val now = currentEpochMs()
        val row = LocalRegistration(
            localId = generateUuidV7(),
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
        q.upsertRegistrationRow(row)
        return row
    }

    override suspend fun upsertFromRemote(remote: List<CommerceRegistration>) {
        remote.forEach { upsertSyncedRemote(it) }
    }

    override suspend fun upsertSyncedRemote(remote: CommerceRegistration) {
        val existing = getByRemoteId(remote.id) ?: getRegistration(remote.id)
        val now = currentEpochMs()
        q.upsertRegistrationRow(
            LocalRegistration(
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
            ),
        )
    }

    override suspend fun setRemoteSynced(
        localId: String,
        remoteId: String,
        registrationStatus: String,
        active: Boolean,
    ) {
        q.setRegistrationRemoteSynced(
            remoteId = remoteId,
            syncStatus = LocalRegistrationSyncStatus.Synced.name,
            registrationStatus = registrationStatus,
            active = active,
            updatedAtEpochMs = currentEpochMs(),
            localId = localId,
        )
    }

    override suspend fun markSyncFailed(localId: String, reason: String) {
        q.markRegistrationSyncFailed(
            syncStatus = LocalRegistrationSyncStatus.SyncFailed.name,
            syncFailureReason = reason,
            updatedAtEpochMs = currentEpochMs(),
            localId = localId,
        )
    }

    override suspend fun getLastRegistrationsSyncEpochMs(): Long? =
        catalog.selectMetadata(KEY_LAST_REG_SYNC).executeAsOneOrNull()?.toLongOrNull()

    override suspend fun setLastRegistrationsSyncEpochMs(epochMs: Long) {
        catalog.upsertMetadata(KEY_LAST_REG_SYNC, epochMs.toString())
    }

    private companion object {
        const val KEY_LAST_REG_SYNC = "lastRegistrationsSync"
    }
}
