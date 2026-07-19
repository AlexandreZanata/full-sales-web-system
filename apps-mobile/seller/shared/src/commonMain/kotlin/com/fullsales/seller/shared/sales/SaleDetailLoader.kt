package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import kotlinx.coroutines.flow.first

class SaleDetailLoader(
    private val apiClient: SellerApiClient,
    private val saleRepository: SaleRepository,
    private val outbox: SyncOutboxRepository? = null,
    private val catalogEnricher: SaleDetailCatalogEnricher = SaleDetailCatalogEnricher(apiClient),
) {
    suspend fun load(
        id: String,
        commerces: List<Commerce>,
        products: List<Product>,
        online: Boolean,
    ): Result<SaleDetailModel> = try {
        val local = saleRepository.getSale(id) ?: saleRepository.getSaleByRemoteId(id)
        val remoteId = local?.remoteId ?: id.takeIf { local == null }
        when {
            online && remoteId != null -> loadRemote(remoteId, local, commerces, products, online)
            local != null -> buildLocalResult(local, commerces, products, online)
            online -> loadRemote(id, null, commerces, products, online)
            else -> Result.failure(IllegalStateException("Sale not available offline"))
        }
    } catch (error: Exception) {
        Result.failure(error)
    }

    private suspend fun loadRemote(
        remoteId: String,
        local: LocalSale?,
        commerces: List<Commerce>,
        products: List<Product>,
        online: Boolean,
    ): Result<SaleDetailModel> = try {
        val remote = apiClient.getSale(remoteId)
        val enriched = enrichForSale(remote.commerceId, remote.items.map { it.productId }, commerces, products, online)
        val pending = hasPendingOutbox(local?.localId)
        val code = remote.displayCode
            ?: local?.displayCode
            ?: displayCodeFor(remoteId, local)
        Result.success(
            buildSaleDetailFromRemote(remote, local, enriched.first, enriched.second, pending, code),
        )
    } catch (error: ApiException) {
        if (error.detail.code == "SALE_NOT_FOUND" && local != null) {
            buildLocalResult(local, commerces, products, online)
        } else {
            Result.failure(error)
        }
    }

    private suspend fun buildLocalResult(
        local: LocalSale,
        commerces: List<Commerce>,
        products: List<Product>,
        online: Boolean,
    ): Result<SaleDetailModel> {
        val enriched = enrichForSale(local.commerceId, local.items.map { it.productId }, commerces, products, online)
        val pending = hasPendingOutbox(local.localId)
        val code = local.displayCode
            ?: displayCodeFor(local.remoteId ?: local.localId, local)
        return Result.success(
            buildSaleDetailFromLocal(local, enriched.first, enriched.second, pending, code),
        )
    }

    private suspend fun displayCodeFor(navigationId: String, local: LocalSale?): String {
        local?.displayCode?.let { return it }
        val all = saleRepository.observeSales().first()
        if (all.isNotEmpty()) {
            return saleDisplayCodeFor(all, navigationId)
        }
        return local?.let { saleDisplayCodeFor(listOf(it), navigationId) } ?: formatSaleDisplayCode(1)
    }

    private suspend fun hasPendingOutbox(localId: String?): Boolean {
        if (localId == null || outbox == null) return false
        return outbox.countPendingForAggregate(localId) > 0
    }

    private suspend fun enrichForSale(
        commerceId: String,
        productIds: List<String>,
        commerces: List<Commerce>,
        products: List<Product>,
        online: Boolean,
    ): Pair<List<Commerce>, List<Product>> =
        catalogEnricher.enrich(commerceId, productIds, commerces, products, online)
}
