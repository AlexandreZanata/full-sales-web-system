package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.LocalRegistration
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import kotlinx.coroutines.flow.Flow

interface RegistrationRepository {
    fun observeRegistrations(): Flow<List<LocalRegistration>>
    suspend fun getRegistration(localId: String): LocalRegistration?
    suspend fun getByRemoteId(remoteId: String): LocalRegistration?
    suspend fun createPending(request: SubmitRegistrationRequest, idempotencyKey: String): LocalRegistration
    suspend fun upsertFromRemote(remote: List<CommerceRegistration>)
    suspend fun upsertSyncedRemote(remote: CommerceRegistration)
    suspend fun setRemoteSynced(localId: String, remoteId: String, registrationStatus: String, active: Boolean)
    suspend fun markSyncFailed(localId: String, reason: String)
    suspend fun getLastRegistrationsSyncEpochMs(): Long?
    suspend fun setLastRegistrationsSyncEpochMs(epochMs: Long)
}
