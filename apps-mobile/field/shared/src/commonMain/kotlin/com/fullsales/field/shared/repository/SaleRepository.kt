package com.fullsales.field.shared.repository

import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.model.Sale

interface SaleRepository {
    suspend fun listSales(): List<Sale>
    suspend fun getSale(localId: String): Sale?
    suspend fun createOfflineSale(localId: String, request: CreateSaleRequest, totalAmount: Double): Sale
    suspend fun updateStatus(localId: String, status: LocalSaleStatus)
    suspend fun setRemoteId(localId: String, remoteId: String, status: LocalSaleStatus)
}
