package com.fullsales.seller.shared.db.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import androidx.room.Transaction
import com.fullsales.seller.shared.db.entity.SaleEntity
import com.fullsales.seller.shared.db.entity.SaleLineEntity
import com.fullsales.seller.shared.db.entity.SaleWithLines
import kotlinx.coroutines.flow.Flow

@Dao
interface SaleDao {
    @Transaction
    @Query("SELECT * FROM sales ORDER BY createdAtEpochMs DESC")
    fun observeSalesWithLines(): Flow<List<SaleWithLines>>

    @Transaction
    @Query("SELECT * FROM sales WHERE localId = :localId LIMIT 1")
    suspend fun getSaleWithLines(localId: String): SaleWithLines?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertSale(sale: SaleEntity)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertLines(lines: List<SaleLineEntity>)

    @Query("UPDATE sales SET status = :status WHERE localId = :localId")
    suspend fun updateStatus(localId: String, status: String)

    @Query(
        "UPDATE sales SET remoteId = :remoteId, status = :status WHERE localId = :localId",
    )
    suspend fun setRemoteId(localId: String, remoteId: String, status: String)

    @Query(
        "UPDATE sales SET status = :status, syncFailureReason = :reason WHERE localId = :localId",
    )
    suspend fun markSyncFailed(localId: String, status: String, reason: String)
}
