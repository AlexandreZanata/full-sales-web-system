package com.fullsales.seller.shared.db.repository

import com.fullsales.seller.shared.db.dao.CatalogDao
import com.fullsales.seller.shared.db.dao.SaleDao
import com.fullsales.seller.shared.db.entity.SaleEntity
import com.fullsales.seller.shared.db.entity.SyncMetadataEntity
import com.fullsales.seller.shared.db.mapper.saleEntity
import com.fullsales.seller.shared.db.mapper.saleLines
import com.fullsales.seller.shared.db.mapper.toModel
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import com.fullsales.seller.shared.model.generateUuidV7
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.sales.toMirroredLocalSale
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class RoomSaleRepository(
    private val dao: SaleDao,
    private val catalogDao: CatalogDao,
) : SaleRepository {
    override fun observeSales(): Flow<List<LocalSale>> =
        dao.observeSalesWithLines().map { rows -> rows.map { it.toModel() } }

    override suspend fun getSale(localId: String): LocalSale? =
        dao.getSaleWithLines(localId)?.toModel()

    override suspend fun getSaleByRemoteId(remoteId: String): LocalSale? =
        dao.getSaleWithLinesByRemoteId(remoteId)?.toModel()

    override suspend fun createLocalSale(
        request: CreateSaleRequest,
        totalAmount: Double,
    ): LocalSale {
        val localId = generateUuidV7()
        val idempotencyKey = generateUuidV7()
        val now = System.currentTimeMillis()
        val items = request.items.map { SaleItem(it.productId, it.quantity) }
        dao.insertSale(
            saleEntity(
                localId, idempotencyKey, request.commerceId, request.paymentMethod,
                totalAmount, LocalSaleStatus.Draft, now,
                origin = SaleOrigin.Local,
            ),
        )
        dao.insertLines(saleLines(localId, items))
        return LocalSale(
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
    }

    override suspend fun updateStatus(localId: String, status: LocalSaleStatus) {
        dao.updateStatus(localId, status.name)
    }

    override suspend fun setRemoteId(localId: String, remoteId: String, status: LocalSaleStatus) {
        dao.setRemoteId(localId, remoteId, status.name)
    }

    override suspend fun markSyncFailed(localId: String, reason: String) {
        dao.markSyncFailed(localId, LocalSaleStatus.SyncFailed.name, reason)
    }

    override suspend fun upsertFromRemoteSales(remoteSales: List<Sale>) {
        remoteSales.forEach { upsertResolvedMirror(it) }
    }

    override suspend fun upsertSyncedRemoteSale(sale: Sale) {
        upsertResolvedMirror(sale)
    }

    override suspend fun getLastSalesSyncEpochMs(): Long? =
        catalogDao.getMetadata(KEY_LAST_SALES_SYNC)?.toLongOrNull()

    override suspend fun setLastSalesSyncEpochMs(epochMs: Long) {
        catalogDao.upsertMetadata(SyncMetadataEntity(KEY_LAST_SALES_SYNC, epochMs.toString()))
    }

    private suspend fun upsertResolvedMirror(remote: Sale) {
        val existing = getSaleByRemoteId(remote.id) ?: getSale(remote.id)
        val mirrored = remote.toMirroredLocalSale(
            existingLocalId = existing?.localId,
            existingOrigin = existing?.origin ?: SaleOrigin.RemoteMirror,
            existingIdempotencyKey = existing?.idempotencyKey,
        )
        dao.upsertSaleWithLines(mirrored.toEntity(), saleLines(mirrored.localId, mirrored.items))
    }

    private fun LocalSale.toEntity() = SaleEntity(
        localId = localId,
        remoteId = remoteId,
        idempotencyKey = idempotencyKey,
        commerceId = commerceId,
        paymentMethod = paymentMethod,
        status = status.name,
        totalAmount = totalAmount,
        totalCurrency = totalCurrency,
        createdAtEpochMs = createdAtEpochMs,
        syncFailureReason = syncFailureReason,
        driverId = driverId,
        origin = origin.name,
    )

    private companion object {
        const val KEY_LAST_SALES_SYNC = "lastSalesSync"
    }
}
