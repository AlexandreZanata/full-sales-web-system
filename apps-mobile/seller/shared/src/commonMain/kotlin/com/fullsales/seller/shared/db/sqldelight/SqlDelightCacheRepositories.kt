package com.fullsales.seller.shared.db.sqldelight

import com.fullsales.seller.shared.media.MediaUrlCacheEntry
import com.fullsales.seller.shared.media.MediaUrlCacheStore
import com.fullsales.seller.shared.model.CommerceAddress
import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.CommerceAddressCache
import com.fullsales.seller.shared.repository.SiteSettingsRepository
import com.fullsales.seller.shared.repository.SiteSettingsSnapshot
import com.fullsales.seller.shared.repository.StockSnapshot
import com.fullsales.seller.shared.repository.StockSnapshotRepository
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class SqlDelightStockSnapshotRepository(
    private val db: SellerLocalDatabase,
) : StockSnapshotRepository {
    private val q get() = db.cacheQueries

    override suspend fun get(productId: String): StockSnapshot? =
        q.selectStock(productId).executeAsOneOrNull()?.let {
            StockSnapshot(it.productId, it.available.toInt(), it.syncedAtEpochMs)
        }

    override suspend fun getAvailableMap(): Map<String, Int> =
        q.selectAllStock().executeAsList().associate { it.productId to it.available.toInt() }

    override suspend fun put(productId: String, available: Int, syncedAtEpochMs: Long) {
        q.upsertStock(productId, available.toLong(), syncedAtEpochMs)
    }

    override suspend fun putAll(balances: Map<String, Int>, syncedAtEpochMs: Long) {
        if (balances.isEmpty()) return
        db.transaction {
            balances.forEach { (id, available) ->
                q.upsertStock(id, available.toLong(), syncedAtEpochMs)
            }
        }
    }
}

class SqlDelightCommerceAddressCache(
    private val db: SellerLocalDatabase,
    private val json: Json = Json { ignoreUnknownKeys = true },
) : CommerceAddressCache {
    private val q get() = db.cacheQueries

    override suspend fun get(commerceId: String): List<CommerceAddress>? {
        val row = q.selectAddresses(commerceId).executeAsOneOrNull() ?: return null
        return runCatching {
            json.decodeFromString<List<CommerceAddress>>(row.addressesJson)
        }.getOrNull()
    }

    override suspend fun put(commerceId: String, addresses: List<CommerceAddress>) {
        q.upsertAddresses(
            commerceId = commerceId,
            addressesJson = json.encodeToString(addresses),
            syncedAtEpochMs = currentEpochMs(),
        )
    }
}

class SqlDelightMediaUrlCacheStore(
    private val db: SellerLocalDatabase,
) : MediaUrlCacheStore {
    private val q get() = db.cacheQueries

    override suspend fun get(fileId: String): MediaUrlCacheEntry? =
        q.selectMediaUrl(fileId).executeAsOneOrNull()?.let {
            MediaUrlCacheEntry(it.fileId, it.url, it.expiresAtEpochMs)
        }

    override suspend fun put(entry: MediaUrlCacheEntry) {
        q.upsertMediaUrl(entry.fileId, entry.url, entry.expiresAtEpochMs)
    }
}

class SqlDelightSiteSettingsRepository(
    private val db: SellerLocalDatabase,
) : SiteSettingsRepository {
    private val q get() = db.cacheQueries

    override suspend fun get(): SiteSettingsSnapshot? {
        val row = q.selectSiteSettings(SINGLETON_ID).executeAsOneOrNull() ?: return null
        return SiteSettingsSnapshot(
            settings = SiteSettings(
                displayName = row.displayName.orEmpty(),
                logoFileId = row.logoFileId,
                logoUrl = row.logoUrl,
                salesContactPhone = row.salesContactPhone,
            ),
            syncedAtEpochMs = row.syncedAtEpochMs,
        )
    }

    override suspend fun upsert(settings: SiteSettings, syncedAtEpochMs: Long) {
        q.upsertSiteSettings(
            id = SINGLETON_ID,
            displayName = settings.displayName,
            logoFileId = settings.logoFileId,
            logoUrl = settings.logoUrl,
            salesContactPhone = settings.salesContactPhone,
            syncedAtEpochMs = syncedAtEpochMs,
        )
    }

    private companion object {
        const val SINGLETON_ID = "default"
    }
}
