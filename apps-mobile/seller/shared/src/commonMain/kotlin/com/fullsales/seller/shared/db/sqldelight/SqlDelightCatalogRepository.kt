package com.fullsales.seller.shared.db.sqldelight

import app.cash.sqldelight.coroutines.asFlow
import app.cash.sqldelight.coroutines.mapToList
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.CatalogRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class SqlDelightCatalogRepository(
    private val db: SellerLocalDatabase,
) : CatalogRepository {
    private val q get() = db.catalogQueries

    override fun observeCommerces(): Flow<List<Commerce>> =
        q.selectActiveCommerces().asFlow().mapToList(Dispatchers.Default)
            .map { rows -> rows.map { it.toModel() } }

    override fun observeProducts(): Flow<List<Product>> =
        q.selectActiveProducts().asFlow().mapToList(Dispatchers.Default)
            .map { rows -> rows.map { it.toModel() } }

    override suspend fun getCommerce(id: String): Commerce? =
        q.selectCommerceById(id).executeAsOneOrNull()?.toModel()

    override suspend fun getProduct(id: String): Product? =
        q.selectProductById(id).executeAsOneOrNull()?.toModel()

    override suspend fun replaceCommerces(commerces: List<Commerce>) {
        db.transaction {
            q.deleteAllCommerces()
            commerces.forEach { q.upsertCommerceRow(it) }
        }
    }

    override suspend fun replaceProducts(products: List<Product>) {
        // Catalog list pull omits detail fields — preserve UOM/description already stored.
        val merged = products.map { incoming ->
            val existing = q.selectProductById(incoming.id).executeAsOneOrNull()
            incoming.copy(
                unitOfMeasure = incoming.unitOfMeasure ?: existing?.unitOfMeasure,
                description = incoming.description ?: existing?.description,
            )
        }
        db.transaction {
            q.deleteAllProducts()
            merged.forEach { q.upsertProductRow(it) }
        }
    }

    override suspend fun upsertProducts(products: List<Product>) {
        if (products.isEmpty()) return
        db.transaction {
            products.forEach { q.upsertProductRow(it) }
        }
    }

    override suspend fun getLastCatalogSyncEpochMs(): Long? =
        q.selectMetadata(KEY_LAST_SYNC).executeAsOneOrNull()?.toLongOrNull()

    override suspend fun setLastCatalogSyncEpochMs(epochMs: Long) {
        q.upsertMetadata(KEY_LAST_SYNC, epochMs.toString())
    }

    private companion object {
        const val KEY_LAST_SYNC = "lastCatalogSync"
    }
}
