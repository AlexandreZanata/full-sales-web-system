package com.fullsales.field.shared.db.repository

import com.fullsales.field.shared.db.dao.SyncOutboxDao
import com.fullsales.field.shared.db.mapper.toEntity
import com.fullsales.field.shared.db.mapper.toModel
import com.fullsales.field.shared.model.SyncOutboxEntry
import com.fullsales.field.shared.repository.SyncOutboxRepository

class RoomSyncOutboxRepository(private val dao: SyncOutboxDao) : SyncOutboxRepository {
    override suspend fun enqueue(entry: SyncOutboxEntry) {
        dao.insert(entry.toEntity())
    }

    override suspend fun listPendingFifo(): List<SyncOutboxEntry> =
        dao.listPendingFifo().map { it.toModel() }

    override suspend fun markCompleted(id: String) {
        dao.markCompleted(id)
    }

    override suspend fun incrementAttempt(id: String, error: String?) {
        dao.incrementAttempt(id, error)
    }

    override suspend fun countPendingForSale(saleLocalId: String): Int =
        dao.countPendingForSale(saleLocalId)
}
