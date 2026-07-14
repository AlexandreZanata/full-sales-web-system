package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import kotlinx.coroutines.flow.Flow

interface SaleRepository {
    fun observeSales(): Flow<List<LocalSale>>
    suspend fun getSale(localId: String): LocalSale?
    suspend fun getSaleByRemoteId(remoteId: String): LocalSale?
    suspend fun createLocalSale(request: CreateSaleRequest, totalAmount: Double): LocalSale
    suspend fun updateStatus(localId: String, status: LocalSaleStatus)
    suspend fun setRemoteId(localId: String, remoteId: String, status: LocalSaleStatus)
    suspend fun markSyncFailed(localId: String, reason: String)
    /** Upsert remote sales pages into LocalStore (OD-16-3 identity). */
    suspend fun upsertFromRemoteSales(remoteSales: List<Sale>)
    /** Persist online create success as Synced LocalStore row. */
    suspend fun upsertSyncedRemoteSale(sale: Sale)
    suspend fun getLastSalesSyncEpochMs(): Long?
    suspend fun setLastSalesSyncEpochMs(epochMs: Long)
}
