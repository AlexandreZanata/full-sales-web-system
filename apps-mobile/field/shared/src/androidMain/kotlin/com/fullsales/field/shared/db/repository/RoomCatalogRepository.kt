package com.fullsales.field.shared.db.repository

import com.fullsales.field.shared.db.dao.CatalogDao
import com.fullsales.field.shared.db.entity.SyncMetadataEntity
import com.fullsales.field.shared.db.mapper.toEntity
import com.fullsales.field.shared.db.mapper.toModel
import com.fullsales.field.shared.model.Commerce
import com.fullsales.field.shared.model.Product
import com.fullsales.field.shared.model.StockBalance
import com.fullsales.field.shared.repository.CatalogRepository

class RoomCatalogRepository(private val dao: CatalogDao) : CatalogRepository {
    override suspend fun listActiveCommerces(): List<Commerce> =
        dao.listActiveCommerces().map { it.toModel() }

    override suspend fun listActiveProducts(): List<Product> =
        dao.listActiveProducts().map { it.toModel() }

    override suspend fun getStockBalance(productId: String): StockBalance? =
        dao.getStockBalance(productId)?.toModel()

    override suspend fun replaceCommerces(commerces: List<Commerce>) {
        dao.clearCommerces()
        dao.upsertCommerces(commerces.map { it.toEntity() })
    }

    override suspend fun replaceProducts(products: List<Product>) {
        dao.clearProducts()
        dao.upsertProducts(products.map { it.toEntity() })
    }

    override suspend fun upsertStockBalance(balance: StockBalance) {
        dao.upsertStockBalance(balance.toEntity())
    }

    override suspend fun listProductIds(): List<String> = dao.listProductIds()

    override suspend fun getLastCatalogSyncEpochMs(): Long? =
        dao.getMetadata(KEY_LAST_SYNC)?.toLongOrNull()

    override suspend fun setLastCatalogSyncEpochMs(epochMs: Long) {
        dao.upsertMetadata(SyncMetadataEntity(KEY_LAST_SYNC, epochMs.toString()))
    }

    private companion object {
        const val KEY_LAST_SYNC = "lastCatalogSync"
    }
}
