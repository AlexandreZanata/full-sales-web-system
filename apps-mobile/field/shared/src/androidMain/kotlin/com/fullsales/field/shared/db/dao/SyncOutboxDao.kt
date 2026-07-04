package com.fullsales.field.shared.db.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.fullsales.field.shared.db.entity.SyncOutboxEntity

@Dao
interface SyncOutboxDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(entry: SyncOutboxEntity)

    @Query(
        "SELECT * FROM sync_outbox WHERE completed = 0 ORDER BY createdAtEpochMs ASC",
    )
    suspend fun listPendingFifo(): List<SyncOutboxEntity>

    @Query("UPDATE sync_outbox SET completed = 1 WHERE id = :id")
    suspend fun markCompleted(id: String)

    @Query(
        "UPDATE sync_outbox SET attempts = attempts + 1, lastError = :error WHERE id = :id",
    )
    suspend fun incrementAttempt(id: String, error: String?)

    @Query(
        "SELECT COUNT(*) FROM sync_outbox WHERE completed = 0 AND saleLocalId = :saleLocalId",
    )
    suspend fun countPendingForSale(saleLocalId: String): Int
}
