package com.fullsales.field.shared.sync

import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.model.SyncOutboxEntry
import com.fullsales.field.shared.repository.SaleRepository
import com.fullsales.field.shared.repository.SyncOutboxRepository
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class OfflineSaleWriter(
    private val sales: SaleRepository,
    private val outbox: SyncOutboxRepository,
    private val json: Json = Json { ignoreUnknownKeys = true },
) {
    suspend fun createSale(localId: String, request: CreateSaleRequest, totalAmount: Double) {
        sales.createOfflineSale(localId, request, totalAmount)
        outbox.enqueue(
            SyncOutboxEntry(
                id = "$localId:create",
                saleLocalId = localId,
                method = "POST",
                path = "/sales",
                bodyJson = json.encodeToString(request),
                idempotencyKey = localId,
                createdAtEpochMs = System.currentTimeMillis(),
            ),
        )
        sales.updateStatus(localId, LocalSaleStatus.PendingSync)
    }
}
