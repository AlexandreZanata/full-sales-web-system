package com.fullsales.seller.shared.db.sqldelight

import app.cash.sqldelight.coroutines.asFlow
import app.cash.sqldelight.coroutines.mapToList
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.model.generateUuidV7
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.sales.toMirroredLocalSale
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class SqlDelightSaleRepository(
    private val db: SellerLocalDatabase,
) : SaleRepository {
    private val sales get() = db.salesQueries
    private val catalog get() = db.catalogQueries

    override fun observeSales(): Flow<List<LocalSale>> =
        sales.selectAllSales().asFlow().mapToList(Dispatchers.Default)
            .map { rows -> rows.map { loadSale(it) } }

    override suspend fun getSale(localId: String): LocalSale? =
        sales.selectSaleByLocalId(localId).executeAsOneOrNull()?.let { loadSale(it) }

    override suspend fun getSaleByRemoteId(remoteId: String): LocalSale? =
        sales.selectSaleByRemoteId(remoteId).executeAsOneOrNull()?.let { loadSale(it) }

    override suspend fun createLocalSale(
        request: CreateSaleRequest,
        totalAmount: Double,
    ): LocalSale {
        val localId = generateUuidV7()
        val idempotencyKey = generateUuidV7()
        val now = currentEpochMs()
        val items = request.items.map { SaleItem(it.productId, it.quantity) }
        val sale = LocalSale(
            localId = localId,
            idempotencyKey = idempotencyKey,
            commerceId = request.commerceId,
            paymentMethod = request.paymentMethod,
            status = LocalSaleStatus.Draft,
            totalAmount = totalAmount,
            items = items,
            createdAtEpochMs = now,
            origin = SaleOrigin.Local,
        )
        upsertSaleWithLines(sale)
        return sale
    }

    override suspend fun updateStatus(localId: String, status: LocalSaleStatus) {
        sales.updateSaleStatus(status.name, localId)
    }

    override suspend fun setRemoteId(
        localId: String,
        remoteId: String,
        status: LocalSaleStatus,
        displayCode: String?,
    ) {
        sales.setSaleRemoteId(remoteId, status.name, displayCode, localId)
    }

    override suspend fun markSyncFailed(localId: String, reason: String) {
        sales.markSaleSyncFailed(LocalSaleStatus.SyncFailed.name, reason, localId)
    }

    override suspend fun upsertFromRemoteSales(remoteSales: List<Sale>) {
        remoteSales.forEach { upsertResolvedMirror(it) }
    }

    override suspend fun upsertSyncedRemoteSale(sale: Sale) {
        upsertResolvedMirror(sale)
    }

    override suspend fun getLastSalesSyncEpochMs(): Long? =
        catalog.selectMetadata(KEY_LAST_SALES_SYNC).executeAsOneOrNull()?.toLongOrNull()

    override suspend fun setLastSalesSyncEpochMs(epochMs: Long) {
        catalog.upsertMetadata(KEY_LAST_SALES_SYNC, epochMs.toString())
    }

    private fun upsertResolvedMirror(remote: Sale) {
        val existing = sales.selectSaleByRemoteId(remote.id).executeAsOneOrNull()
            ?: sales.selectSaleByLocalId(remote.id).executeAsOneOrNull()
        val mirrored = remote.toMirroredLocalSale(
            existingLocalId = existing?.localId,
            existingOrigin = existing?.let {
                runCatching { SaleOrigin.valueOf(it.origin) }.getOrDefault(SaleOrigin.RemoteMirror)
            } ?: SaleOrigin.RemoteMirror,
            existingIdempotencyKey = existing?.idempotencyKey,
            existingDisplayCode = existing?.displayCode,
        )
        upsertSaleWithLines(mirrored)
    }

    private fun upsertSaleWithLines(sale: LocalSale) {
        db.transaction {
            sales.deleteLinesForSale(sale.localId)
            sales.upsertSaleRow(sale)
            sales.insertSaleItems(sale.localId, sale.items)
        }
    }

    private fun loadSale(row: Sales): LocalSale {
        val lines = sales.selectLinesForSale(row.localId).executeAsList()
        return row.toLocalSale(lines)
    }

    private companion object {
        const val KEY_LAST_SALES_SYNC = "lastSalesSync"
    }
}
