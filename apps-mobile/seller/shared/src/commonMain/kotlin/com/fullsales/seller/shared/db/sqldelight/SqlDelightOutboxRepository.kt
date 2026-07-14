package com.fullsales.seller.shared.db.sqldelight

import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.repository.SyncOutboxRepository

class SqlDelightOutboxRepository(
    private val db: SellerLocalDatabase,
) : SyncOutboxRepository {
    private val q get() = db.outboxQueries

    override suspend fun enqueue(entry: SyncOutboxEntry) {
        q.insertOutboxRow(entry)
    }

    override suspend fun listPendingFifo(): List<SyncOutboxEntry> =
        q.listPendingFifo().executeAsList().map { it.toModel() }

    override suspend fun getEntry(id: String): SyncOutboxEntry? =
        q.selectOutboxById(id).executeAsOneOrNull()?.toModel()

    override suspend fun markCompleted(id: String) {
        q.markOutboxCompleted(id)
    }

    override suspend fun markFailed(id: String, error: String?) {
        q.markOutboxFailed(error, id)
    }

    override suspend fun countPendingForAggregate(aggregateId: String): Int =
        q.countPendingForAggregate(aggregateId).executeAsOne().toInt()
}
