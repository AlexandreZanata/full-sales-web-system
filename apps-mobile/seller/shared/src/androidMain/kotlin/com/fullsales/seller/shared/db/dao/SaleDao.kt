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

    @Transaction
    @Query("SELECT * FROM sales WHERE remoteId = :remoteId LIMIT 1")
    suspend fun getSaleWithLinesByRemoteId(remoteId: String): SaleWithLines?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertSale(sale: SaleEntity)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertLines(lines: List<SaleLineEntity>)

    @Query("DELETE FROM sale_lines WHERE saleLocalId = :saleLocalId")
    suspend fun deleteLines(saleLocalId: String)

    @Transaction
    suspend fun upsertSaleWithLines(sale: SaleEntity, lines: List<SaleLineEntity>) {
        insertSale(sale)
        deleteLines(sale.localId)
        if (lines.isNotEmpty()) insertLines(lines)
    }

    @Query("UPDATE sales SET status = :status WHERE localId = :localId")
    suspend fun updateStatus(localId: String, status: String)

    @Query(
        """
        UPDATE sales
        SET remoteId = :remoteId,
            status = :status,
            displayCode = COALESCE(:displayCode, displayCode)
        WHERE localId = :localId
        """,
    )
    suspend fun setRemoteId(
        localId: String,
        remoteId: String,
        status: String,
        displayCode: String? = null,
    )

    @Query(
        "UPDATE sales SET status = :status, syncFailureReason = :reason WHERE localId = :localId",
    )
    suspend fun markSyncFailed(localId: String, status: String, reason: String)
}
