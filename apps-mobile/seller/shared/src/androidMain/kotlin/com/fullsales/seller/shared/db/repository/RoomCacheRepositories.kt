package com.fullsales.seller.shared.db.repository

import com.fullsales.seller.shared.db.dao.CacheDao
import com.fullsales.seller.shared.db.entity.CommerceAddressCacheEntity
import com.fullsales.seller.shared.db.entity.StockSnapshotEntity
import com.fullsales.seller.shared.model.CommerceAddress
import com.fullsales.seller.shared.repository.CommerceAddressCache
import com.fullsales.seller.shared.repository.StockSnapshot
import com.fullsales.seller.shared.repository.StockSnapshotRepository
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class RoomStockSnapshotRepository(
    private val dao: CacheDao,
) : StockSnapshotRepository {
    override suspend fun get(productId: String): StockSnapshot? =
        dao.getStock(productId)?.let { StockSnapshot(it.productId, it.available, it.syncedAtEpochMs) }

    override suspend fun getAvailableMap(): Map<String, Int> =
        dao.listStock().associate { it.productId to it.available }

    override suspend fun put(productId: String, available: Int, syncedAtEpochMs: Long) {
        dao.upsertStock(StockSnapshotEntity(productId, available, syncedAtEpochMs))
    }

    override suspend fun putAll(balances: Map<String, Int>, syncedAtEpochMs: Long) {
        if (balances.isEmpty()) return
        dao.upsertStockAll(
            balances.map { (id, available) -> StockSnapshotEntity(id, available, syncedAtEpochMs) },
        )
    }
}

class RoomCommerceAddressCache(
    private val dao: CacheDao,
    private val json: Json = Json { ignoreUnknownKeys = true },
) : CommerceAddressCache {
    override suspend fun get(commerceId: String): List<CommerceAddress>? {
        val row = dao.getAddresses(commerceId) ?: return null
        return runCatching {
            json.decodeFromString<List<CommerceAddress>>(row.addressesJson)
        }.getOrNull()
    }

    override suspend fun put(commerceId: String, addresses: List<CommerceAddress>) {
        dao.upsertAddresses(
            CommerceAddressCacheEntity(
                commerceId = commerceId,
                addressesJson = json.encodeToString(addresses),
                syncedAtEpochMs = com.fullsales.seller.shared.model.currentEpochMs(),
            ),
        )
    }
}
