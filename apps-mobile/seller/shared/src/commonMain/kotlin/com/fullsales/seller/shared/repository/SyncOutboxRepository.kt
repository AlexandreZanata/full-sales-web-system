package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.SyncOutboxEntry

interface SyncOutboxRepository {
    suspend fun enqueue(entry: SyncOutboxEntry)
    suspend fun listPendingFifo(): List<SyncOutboxEntry>
    suspend fun markCompleted(id: String)
    suspend fun markFailed(id: String, error: String?)
    suspend fun countPendingForSale(saleLocalId: String): Int
}
