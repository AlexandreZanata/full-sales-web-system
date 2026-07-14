package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import com.fullsales.seller.shared.sync.SellerSyncCoordinator

sealed class SaleActionResult {
    data object Success : SaleActionResult()
    data class Failure(val code: String, val message: String) : SaleActionResult()
}

class SaleActionSubmitter(
    private val apiClient: SellerApiClient,
    private val saleRepository: SaleRepository,
    private val outbox: SyncOutboxRepository,
    private val syncCoordinator: SellerSyncCoordinator? = null,
) {
    suspend fun confirm(detail: SaleDetailModel, online: Boolean): SaleActionResult =
        runAction(detail, online, "confirm") { id -> apiClient.confirmSale(id) }

    suspend fun cancel(detail: SaleDetailModel, online: Boolean): SaleActionResult =
        runAction(detail, online, "cancel") { id -> apiClient.cancelSale(id) }

    private suspend fun runAction(
        detail: SaleDetailModel,
        online: Boolean,
        action: String,
        call: suspend (String) -> Unit,
    ): SaleActionResult {
        val localId = detail.localId ?: detail.remoteId
            ?: return SaleActionResult.Failure("NO_REMOTE_ID", "NO_REMOTE_ID")
        val remoteId = detail.remoteId
        if (remoteId != null) {
            if (!online) return enqueueOptimistic(localId, pathSaleId = remoteId, action, dependsOn = null)
            return runCatching {
                call(remoteId)
                updateLocalStatus(localId, action)
                syncCoordinator?.pushOutbox()
                SaleActionResult.Success
            }.getOrElse { error ->
                if (isTransportFailure(error)) {
                    enqueueOptimistic(localId, pathSaleId = remoteId, action, dependsOn = null)
                } else {
                    mapError(error)
                }
            }
        }
        val createId = "$localId:create"
        val pendingCreate = outbox.getEntry(createId)?.takeUnless { it.completed }
            ?: return SaleActionResult.Failure("NO_REMOTE_ID", "NO_REMOTE_ID")
        // OD-16-2: chain confirm/cancel after pending create (path uses localId until sync resolves remoteId).
        return enqueueOptimistic(localId, pathSaleId = localId, action, dependsOn = pendingCreate.id)
    }

    private suspend fun enqueueOptimistic(
        localId: String,
        pathSaleId: String,
        action: String,
        dependsOn: String?,
    ): SaleActionResult {
        outbox.enqueue(
            SyncOutboxEntry(
                id = "$localId:$action",
                aggregateId = localId,
                method = "POST",
                path = "/sales/$pathSaleId/$action",
                bodyJson = "{}",
                idempotencyKey = "$localId:$action",
                createdAtEpochMs = currentEpochMs(),
                entityType = SyncEntityType.Sale,
                dependsOnOutboxId = dependsOn,
            ),
        )
        updateLocalStatus(localId, action)
        return SaleActionResult.Success
    }

    private suspend fun updateLocalStatus(localId: String, action: String) {
        val status = when (action) {
            "confirm" -> LocalSaleStatus.Confirmed
            else -> LocalSaleStatus.Cancelled
        }
        saleRepository.updateStatus(localId, status)
    }

    private fun mapError(error: Throwable): SaleActionResult.Failure {
        if (error is ApiException) {
            return SaleActionResult.Failure(
                code = error.detail.code,
                message = error.detail.code,
            )
        }
        return SaleActionResult.Failure("NETWORK_ERROR", "NETWORK_ERROR")
    }
}
