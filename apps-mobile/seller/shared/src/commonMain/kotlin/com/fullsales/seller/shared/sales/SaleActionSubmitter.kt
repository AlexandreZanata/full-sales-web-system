package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.LocalSaleStatus
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
        val remoteId = detail.remoteId
            ?: return SaleActionResult.Failure("NO_REMOTE_ID", "Sync sale before changing status")
        val localId = detail.localId
        return if (online) {
            runCatching {
                call(remoteId)
                localId?.let { updateLocalStatus(it, action) }
                syncCoordinator?.syncPullAndPush()
                SaleActionResult.Success
            }.getOrElse { mapError(it) }
        } else {
            enqueueOffline(localId ?: remoteId, remoteId, action)
            SaleActionResult.Success
        }
    }

    private suspend fun enqueueOffline(localId: String, remoteId: String, action: String) {
        outbox.enqueue(
            SyncOutboxEntry(
                id = "$localId:$action",
                saleLocalId = localId,
                method = "POST",
                path = "/sales/$remoteId/$action",
                bodyJson = "{}",
                idempotencyKey = "$localId:$action",
                createdAtEpochMs = currentEpochMs(),
            ),
        )
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
                message = mapSaleActionError(error.detail.code),
            )
        }
        return SaleActionResult.Failure("NETWORK_ERROR", mapSaleActionError("NETWORK_ERROR"))
    }
}
