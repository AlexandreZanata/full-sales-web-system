package com.fullsales.field.shared.repository

import com.fullsales.field.shared.model.SyncOutboxEntry

interface SyncOutboxRepository {
    suspend fun enqueue(entry: SyncOutboxEntry)
    suspend fun listPendingFifo(): List<SyncOutboxEntry>
    suspend fun markCompleted(id: String)
    suspend fun incrementAttempt(id: String, error: String?)
    suspend fun countPendingForSale(saleLocalId: String): Int
}
