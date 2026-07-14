package com.fullsales.seller.shared.db.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.fullsales.seller.shared.db.entity.RegistrationEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface RegistrationDao {
    @Query("SELECT * FROM registrations ORDER BY createdAtEpochMs DESC")
    fun observeAll(): Flow<List<RegistrationEntity>>

    @Query("SELECT * FROM registrations WHERE localId = :localId LIMIT 1")
    suspend fun getByLocalId(localId: String): RegistrationEntity?

    @Query("SELECT * FROM registrations WHERE remoteId = :remoteId LIMIT 1")
    suspend fun getByRemoteId(remoteId: String): RegistrationEntity?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: RegistrationEntity)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsertAll(entities: List<RegistrationEntity>)

    @Query(
        "UPDATE registrations SET syncStatus = :syncStatus, syncFailureReason = :reason, " +
            "updatedAtEpochMs = :updatedAt WHERE localId = :localId",
    )
    suspend fun markSyncFailed(
        localId: String,
        syncStatus: String,
        reason: String,
        updatedAt: Long,
    )

    @Query(
        "UPDATE registrations SET remoteId = :remoteId, syncStatus = :syncStatus, " +
            "registrationStatus = :registrationStatus, active = :active, " +
            "updatedAtEpochMs = :updatedAt WHERE localId = :localId",
    )
    suspend fun setRemoteSynced(
        localId: String,
        remoteId: String,
        syncStatus: String,
        registrationStatus: String,
        active: Boolean,
        updatedAt: Long,
    )
}
