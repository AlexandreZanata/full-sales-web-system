package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.currentEpochMs

data class StockSnapshot(
    val productId: String,
    val available: Int,
    val syncedAtEpochMs: Long,
)

/** Last-known stock balances; no hard TTL (OD-14-3). */
interface StockSnapshotRepository {
    suspend fun get(productId: String): StockSnapshot?
    suspend fun getAvailableMap(): Map<String, Int>
    suspend fun put(productId: String, available: Int, syncedAtEpochMs: Long = currentEpochMs())
    suspend fun putAll(balances: Map<String, Int>, syncedAtEpochMs: Long = currentEpochMs())
}
