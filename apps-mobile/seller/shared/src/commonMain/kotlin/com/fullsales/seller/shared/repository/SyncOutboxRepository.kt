package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.SyncOutboxEntry

interface SyncOutboxRepository {
    suspend fun enqueue(entry: SyncOutboxEntry)
    suspend fun listPendingFifo(): List<SyncOutboxEntry>
    suspend fun getEntry(id: String): SyncOutboxEntry?
    suspend fun markCompleted(id: String)
    suspend fun markFailed(id: String, error: String?)
    suspend fun countPendingForAggregate(aggregateId: String): Int
}
