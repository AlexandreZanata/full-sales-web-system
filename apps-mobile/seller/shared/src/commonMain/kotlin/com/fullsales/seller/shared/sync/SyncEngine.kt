package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository

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
                sales.markSyncFailed(
                    entry.saleLocalId,
                    result.errorCode ?: "INSUFFICIENT_STOCK",
                )
                false
            }
            SyncHttpOutcome.NetworkError -> {
                outbox.markFailed(entry.id, result.errorCode)
                true
            }
            SyncHttpOutcome.Unauthorized -> {
                outbox.markFailed(entry.id, "UNAUTHORIZED")
                true
            }
            SyncHttpOutcome.ClientError -> {
                outbox.markFailed(entry.id, result.errorCode)
                false
            }
        }
    }

    private suspend fun applySuccess(entry: SyncOutboxEntry, remoteId: String?) {
        when {
            entry.path == "/sales" && remoteId != null ->
                sales.setRemoteId(entry.saleLocalId, remoteId, LocalSaleStatus.Synced)
            entry.path.endsWith("/confirm") ->
                sales.updateStatus(entry.saleLocalId, LocalSaleStatus.Confirmed)
            entry.path.endsWith("/cancel") ->
                sales.updateStatus(entry.saleLocalId, LocalSaleStatus.Cancelled)
        }
    }
}
