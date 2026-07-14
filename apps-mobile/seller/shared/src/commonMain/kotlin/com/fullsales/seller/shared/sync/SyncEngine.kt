package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.repository.RegistrationRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository

class SyncEngine(
    private val outbox: SyncOutboxRepository,
    private val sales: SaleRepository,
    private val transport: SyncTransport,
    private val tokenRefresher: SyncTokenRefresher,
    private val registrations: RegistrationRepository? = null,
    private val maxAttempts: Int = 5,
) {
    suspend fun processOutbox(): SyncProcessResult {
        var processed = 0
        for (entry in outbox.listPendingFifo()) {
            if (entry.attempts >= maxAttempts) {
                deadLetter(entry)
                continue
            }
            if (isBlockedByDependency(entry)) continue
            val stop = processEntry(resolveRemotePath(entry))
            if (stop) return SyncProcessResult(processed, stoppedEarly = true)
            processed++
        }
        return SyncProcessResult(processed)
    }

    private suspend fun isBlockedByDependency(entry: SyncOutboxEntry): Boolean {
        val parentId = entry.dependsOnOutboxId ?: return false
        val parent = outbox.getEntry(parentId) ?: return false
        return !parent.completed
    }

    private suspend fun resolveRemotePath(entry: SyncOutboxEntry): SyncOutboxEntry {
        if (entry.entityType != SyncEntityType.Sale) return entry
        if (entry.path == "/sales") return entry
        if (!entry.path.endsWith("/confirm") && !entry.path.endsWith("/cancel")) return entry
        val remoteId = sales.getSale(entry.aggregateId)?.remoteId ?: return entry
        val action = if (entry.path.endsWith("/confirm")) "confirm" else "cancel"
        return entry.copy(path = "/sales/$remoteId/$action")
    }

    private suspend fun deadLetter(entry: SyncOutboxEntry) {
        when (entry.entityType) {
            SyncEntityType.Registration ->
                registrations?.markSyncFailed(entry.aggregateId, entry.lastError ?: "MAX_ATTEMPTS")
            else ->
                sales.markSyncFailed(entry.aggregateId, entry.lastError ?: "MAX_ATTEMPTS")
        }
        outbox.markCompleted(entry.id)
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
                sales.markSyncFailed(entry.aggregateId, result.errorCode ?: "INSUFFICIENT_STOCK")
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
            entry.path == "/commerces/registrations" && remoteId != null ->
                registrations?.setRemoteSynced(
                    localId = entry.aggregateId,
                    remoteId = remoteId,
                    registrationStatus = "PendingReview",
                    active = false,
                )
            entry.path == "/sales" && remoteId != null ->
                sales.setRemoteId(entry.aggregateId, remoteId, LocalSaleStatus.Synced)
            entry.path.endsWith("/confirm") ->
                sales.updateStatus(entry.aggregateId, LocalSaleStatus.Confirmed)
            entry.path.endsWith("/cancel") ->
                sales.updateStatus(entry.aggregateId, LocalSaleStatus.Cancelled)
        }
    }
}
