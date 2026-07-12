package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.CommerceAddress
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class MemoryStockSnapshotRepository : StockSnapshotRepository {
    private val mutex = Mutex()
    private val snapshots = linkedMapOf<String, StockSnapshot>()

    override suspend fun get(productId: String): StockSnapshot? = mutex.withLock { snapshots[productId] }

    override suspend fun getAvailableMap(): Map<String, Int> = mutex.withLock {
        snapshots.mapValues { it.value.available }
    }

    override suspend fun put(productId: String, available: Int, syncedAtEpochMs: Long) = mutex.withLock {
        snapshots[productId] = StockSnapshot(productId, available, syncedAtEpochMs)
    }

    override suspend fun putAll(balances: Map<String, Int>, syncedAtEpochMs: Long) = mutex.withLock {
        balances.forEach { (id, available) ->
            snapshots[id] = StockSnapshot(id, available, syncedAtEpochMs)
        }
    }
}

class MemoryCommerceAddressCache : CommerceAddressCache {
    private val mutex = Mutex()
    private val byCommerce = linkedMapOf<String, List<CommerceAddress>>()

    override suspend fun get(commerceId: String): List<CommerceAddress>? = mutex.withLock {
        byCommerce[commerceId]
    }

    override suspend fun put(commerceId: String, addresses: List<CommerceAddress>) = mutex.withLock {
        byCommerce[commerceId] = addresses
    }
}
