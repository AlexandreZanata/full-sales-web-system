package com.fullsales.field.shared.db.repository

import com.fullsales.field.shared.db.dao.SaleDao
import com.fullsales.field.shared.db.mapper.saleEntity
import com.fullsales.field.shared.db.mapper.saleLines
import com.fullsales.field.shared.db.mapper.toModel
import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.model.Sale
import com.fullsales.field.shared.model.SaleItem
import com.fullsales.field.shared.repository.SaleRepository

class RoomSaleRepository(private val dao: SaleDao) : SaleRepository {
    override suspend fun listSales(): List<Sale> =
        dao.listSalesWithLines().map { it.toModel() }

    override suspend fun getSale(localId: String): Sale? =
        dao.getSaleWithLines(localId)?.toModel()

    override suspend fun createOfflineSale(
        localId: String,
        request: CreateSaleRequest,
        totalAmount: Double,
    ): Sale {
        val now = System.currentTimeMillis()
        val items = request.items.map { SaleItem(it.productId, it.quantity) }
        dao.insertSale(
            saleEntity(localId, request.commerceId, request.paymentMethod, totalAmount, LocalSaleStatus.DraftLocal, now),
        )
        dao.insertLines(saleLines(localId, items))
        return Sale(
            localId = localId,
            commerceId = request.commerceId,
            status = LocalSaleStatus.DraftLocal,
            paymentMethod = request.paymentMethod,
            totalAmount = totalAmount,
            totalCurrency = "BRL",
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
