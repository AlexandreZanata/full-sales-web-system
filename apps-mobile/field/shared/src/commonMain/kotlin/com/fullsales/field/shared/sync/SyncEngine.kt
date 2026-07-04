package com.fullsales.field.shared.sync

import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.model.SyncOutboxEntry
import com.fullsales.field.shared.repository.SaleRepository
import com.fullsales.field.shared.repository.SyncOutboxRepository

class SyncEngine(
    private val outbox: SyncOutboxRepository,
    private val sales: SaleRepository,
    private val transport: SyncTransport,
    private val tokenRefresher: SyncTokenRefresher,
    private val maxAttempts: Int = 5,
) {
    suspend fun processOutbox(): SyncProcessResult {
        var processed = 0
        for (entry in outbox.listPendingFifo()) {
            if (entry.attempts >= maxAttempts) continue
            val stop = processEntry(entry)
            if (stop) return SyncProcessResult(processed, stoppedEarly = true)
            processed++
        }
        return SyncProcessResult(processed)
    }

    private suspend fun processEntry(entry: SyncOutboxEntry): Boolean {
        var result = transport.execute(entry)
        if (result.outcome == SyncHttpOutcome.Unauthorized) {
            if (!tokenRefresher.refreshToken()) return true
            result = transport.execute(entry)
        }
        return when (result.outcome) {
            SyncHttpOutcome.Success -> {
                outbox.markCompleted(entry.id)
                applySuccess(entry, result.remoteId)
                false
            }
            SyncHttpOutcome.InsufficientStock -> {
                outbox.markCompleted(entry.id)
                sales.updateStatus(entry.saleLocalId, LocalSaleStatus.SyncFailed)
                false
            }
            SyncHttpOutcome.NetworkError -> {
                outbox.incrementAttempt(entry.id, result.errorCode)
                true
            }
            SyncHttpOutcome.Unauthorized -> {
                outbox.incrementAttempt(entry.id, "UNAUTHORIZED")
                true
            }
            SyncHttpOutcome.ClientError -> {
                outbox.incrementAttempt(entry.id, result.errorCode)
                false
            }
        }
    }

    private suspend fun applySuccess(entry: SyncOutboxEntry, remoteId: String?) {
        if (entry.path == "/sales" && remoteId != null) {
            sales.setRemoteId(entry.saleLocalId, remoteId, LocalSaleStatus.PendingRemote)
        } else if (entry.path.endsWith("/confirm")) {
            sales.updateStatus(entry.saleLocalId, LocalSaleStatus.Confirmed)
        } else if (entry.path.endsWith("/cancel")) {
            sales.updateStatus(entry.saleLocalId, LocalSaleStatus.Cancelled)
        }
    }
}
