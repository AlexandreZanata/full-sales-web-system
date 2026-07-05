package com.fullsales.seller.shared.db.repository

import com.fullsales.seller.shared.db.dao.SyncOutboxDao
import com.fullsales.seller.shared.db.mapper.toEntity
import com.fullsales.seller.shared.db.mapper.toModel
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.repository.SyncOutboxRepository

class RoomSyncOutboxRepository(private val dao: SyncOutboxDao) : SyncOutboxRepository {
    override suspend fun enqueue(entry: SyncOutboxEntry) {
        dao.insert(entry.toEntity())
    }

    override suspend fun listPendingFifo(): List<SyncOutboxEntry> =
        dao.listPendingFifo().map { it.toModel() }

    override suspend fun markCompleted(id: String) {
        dao.markCompleted(id)
    }

    override suspend fun markFailed(id: String, error: String?) {
        dao.markFailed(id, error)
    }

    override suspend fun countPendingForSale(saleLocalId: String): Int =
        dao.countPendingForSale(saleLocalId)
}
