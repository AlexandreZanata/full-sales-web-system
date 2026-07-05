package com.fullsales.seller.shared.db.repository

import com.fullsales.seller.shared.db.dao.CatalogDao
import com.fullsales.seller.shared.db.mapper.toEntity
import com.fullsales.seller.shared.db.mapper.toModel
import com.fullsales.seller.shared.db.entity.SyncMetadataEntity
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.CatalogRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class RoomCatalogRepository(private val dao: CatalogDao) : CatalogRepository {
    override fun observeCommerces(): Flow<List<Commerce>> =
        dao.observeActiveCommerces().map { rows -> rows.map { it.toModel() } }

    override fun observeProducts(): Flow<List<Product>> =
        dao.observeActiveProducts().map { rows -> rows.map { it.toModel() } }

    override suspend fun replaceCommerces(commerces: List<Commerce>) {
        dao.replaceCommerces(commerces.map { it.toEntity() })
    }

    override suspend fun replaceProducts(products: List<Product>) {
        dao.replaceProducts(products.map { it.toEntity() })
    }

    override suspend fun getLastCatalogSyncEpochMs(): Long? =
        dao.getMetadata(KEY_LAST_SYNC)?.toLongOrNull()

    override suspend fun setLastCatalogSyncEpochMs(epochMs: Long) {
        dao.upsertMetadata(SyncMetadataEntity(KEY_LAST_SYNC, epochMs.toString()))
    }

    private companion object {
        const val KEY_LAST_SYNC = "lastCatalogSync"
    }
}
