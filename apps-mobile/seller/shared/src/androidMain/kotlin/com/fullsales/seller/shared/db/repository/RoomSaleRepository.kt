package com.fullsales.seller.shared.db.repository

import com.fullsales.seller.shared.db.dao.SaleDao
import com.fullsales.seller.shared.db.mapper.saleEntity
import com.fullsales.seller.shared.db.mapper.saleLines
import com.fullsales.seller.shared.db.mapper.toModel
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.generateUuidV7
import com.fullsales.seller.shared.repository.SaleRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class RoomSaleRepository(private val dao: SaleDao) : SaleRepository {
    override fun observeSales(): Flow<List<LocalSale>> =
        dao.observeSalesWithLines().map { rows -> rows.map { it.toModel() } }

    override suspend fun getSale(localId: String): LocalSale? =
        dao.getSaleWithLines(localId)?.toModel()

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
        )
    }

    override suspend fun updateStatus(localId: String, status: LocalSaleStatus) {
        dao.updateStatus(localId, status.name)
    }

    override suspend fun setRemoteId(localId: String, remoteId: String, status: LocalSaleStatus) {
        dao.setRemoteId(localId, remoteId, status.name)
    }
}
