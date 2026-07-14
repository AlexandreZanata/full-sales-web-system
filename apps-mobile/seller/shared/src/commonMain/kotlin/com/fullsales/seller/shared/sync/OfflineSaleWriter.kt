package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class OfflineSaleWriter(
    private val sales: SaleRepository,
    private val outbox: SyncOutboxRepository,
    private val json: Json = Json { ignoreUnknownKeys = true },
) {
    suspend fun createSale(request: CreateSaleRequest, totalAmount: Double): LocalSale {
        val sale = sales.createLocalSale(request, totalAmount)
        outbox.enqueue(
            SyncOutboxEntry(
                id = "${sale.localId}:create",
                aggregateId = sale.localId,
                method = "POST",
                path = "/sales",
                bodyJson = json.encodeToString(request),
                idempotencyKey = sale.idempotencyKey,
                createdAtEpochMs = sale.createdAtEpochMs,
                entityType = SyncEntityType.Sale,
            ),
        )
        sales.updateStatus(sale.localId, LocalSaleStatus.PendingSync)
        return sales.getSale(sale.localId) ?: sale.copy(status = LocalSaleStatus.PendingSync)
    }
}
