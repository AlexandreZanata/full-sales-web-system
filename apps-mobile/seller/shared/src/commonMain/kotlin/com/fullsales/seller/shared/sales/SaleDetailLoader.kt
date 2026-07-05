package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.SaleRepository

class SaleDetailLoader(
    private val apiClient: SellerApiClient,
    private val saleRepository: SaleRepository,
) {
    suspend fun load(
        id: String,
        commerces: List<Commerce>,
        products: List<Product>,
        online: Boolean,
    ): Result<SaleDetailModel> = try {
        val local = saleRepository.getSale(id) ?: saleRepository.getSaleByRemoteId(id)
        val remoteId = local?.remoteId ?: id.takeIf { local == null }
        if (online && remoteId != null) {
            loadRemote(remoteId, local, commerces, products)
        } else if (local != null) {
            Result.success(buildSaleDetailFromLocal(local, commerces, products))
        } else if (online) {
            loadRemote(id, null, commerces, products)
        } else {
            Result.failure(IllegalStateException("Sale not available offline"))
        }
    } catch (error: Exception) {
        Result.failure(error)
    }

    private suspend fun loadRemote(
        remoteId: String,
        local: com.fullsales.seller.shared.model.LocalSale?,
        commerces: List<Commerce>,
        products: List<Product>,
    ): Result<SaleDetailModel> = try {
        val remote = apiClient.getSale(remoteId)
        Result.success(buildSaleDetailFromRemote(remote, local, commerces, products))
    } catch (error: ApiException) {
        if (error.detail.code == "SALE_NOT_FOUND" && local != null) {
            Result.success(buildSaleDetailFromLocal(local, commerces, products))
        } else {
            Result.failure(error)
        }
    }
}
