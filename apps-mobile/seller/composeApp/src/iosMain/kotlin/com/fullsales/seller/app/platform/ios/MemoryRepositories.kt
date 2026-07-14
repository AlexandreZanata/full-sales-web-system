package com.fullsales.seller.app.platform.ios

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import com.fullsales.seller.shared.sales.toMirroredLocalSale
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

// ponytail: Phase 66 may add SQLDelight; in-memory cache for iOS MVP shell.
internal class MemoryCatalogRepository : CatalogRepository {
    private val commerces = mutableListOf<Commerce>()
    private val products = mutableListOf<Product>()
    private var lastSync: Long? = null
    private val commerceFlow = MutableStateFlow<List<Commerce>>(emptyList())
    private val productFlow = MutableStateFlow<List<Product>>(emptyList())

    override fun observeCommerces(): Flow<List<Commerce>> = commerceFlow.asStateFlow()

    override fun observeProducts(): Flow<List<Product>> = productFlow.asStateFlow()

    override suspend fun getCommerce(id: String): Commerce? = commerces.firstOrNull { it.id == id }

    override suspend fun getProduct(id: String): Product? = products.firstOrNull { it.id == id }

    override suspend fun replaceCommerces(items: List<Commerce>) {
        commerces.clear()
        commerces.addAll(items)
        commerceFlow.value = commerces.filter { it.active }
    }

    override suspend fun replaceProducts(items: List<Product>) {
        val preserved = products.associateBy { it.id }
        products.clear()
        products.addAll(
            items.map { incoming ->
                val old = preserved[incoming.id]
                incoming.copy(
                    unitOfMeasure = incoming.unitOfMeasure ?: old?.unitOfMeasure,
                    description = incoming.description ?: old?.description,
                )
            },
        )
        productFlow.value = products.filter { it.active }
    }

    override suspend fun upsertProducts(items: List<Product>) {
        items.forEach { incoming ->
            val idx = products.indexOfFirst { it.id == incoming.id }
            if (idx >= 0) products[idx] = incoming else products.add(incoming)
        }
        productFlow.value = products.filter { it.active }
    }

    override suspend fun getLastCatalogSyncEpochMs(): Long? = lastSync

    override suspend fun setLastCatalogSyncEpochMs(epochMs: Long) {
        lastSync = epochMs
    }
}

internal class MemorySaleRepository : SaleRepository {
    private val mutex = Mutex()
    private val sales = linkedMapOf<String, LocalSale>()
    private val flow = MutableStateFlow<List<LocalSale>>(emptyList())

    override fun observeSales(): Flow<List<LocalSale>> = flow.asStateFlow()

    override suspend fun getSale(localId: String): LocalSale? = mutex.withLock { sales[localId] }

    override suspend fun getSaleByRemoteId(remoteId: String): LocalSale? = mutex.withLock {
        sales.values.firstOrNull { it.remoteId == remoteId }
    }

    override suspend fun createLocalSale(request: CreateSaleRequest, totalAmount: Double): LocalSale =
        mutex.withLock {
            val localId = "local-${sales.size + 1}"
            val sale = LocalSale(
                localId = localId,
                idempotencyKey = "idem-$localId",
                commerceId = request.commerceId,
                paymentMethod = request.paymentMethod,
                status = LocalSaleStatus.Draft,
                totalAmount = totalAmount,
                items = request.items.map { SaleItem(it.productId, it.quantity) },
                createdAtEpochMs = 1L,
                origin = com.fullsales.seller.shared.model.SaleOrigin.Local,
            )
            sales[localId] = sale
            flow.value = sales.values.toList()
            sale
        }

    override suspend fun updateStatus(localId: String, status: LocalSaleStatus) {
        mutex.withLock {
            sales[localId]?.let { sales[localId] = it.copy(status = status) }
            flow.value = sales.values.toList()
        }
    }

    override suspend fun setRemoteId(localId: String, remoteId: String, status: LocalSaleStatus) {
        mutex.withLock {
            sales[localId]?.let { sales[localId] = it.copy(remoteId = remoteId, status = status) }
            flow.value = sales.values.toList()
        }
    }

    override suspend fun markSyncFailed(localId: String, reason: String) {
        mutex.withLock {
            sales[localId]?.let {
                sales[localId] = it.copy(status = LocalSaleStatus.SyncFailed, syncFailureReason = reason)
            }
            flow.value = sales.values.toList()
        }
    }

    override suspend fun upsertFromRemoteSales(remoteSales: List<com.fullsales.seller.shared.model.Sale>) {
        mutex.withLock {
            remoteSales.forEach { remote ->
                val existing = sales.values.firstOrNull { it.remoteId == remote.id } ?: sales[remote.id]
                val mirrored = remote.toMirroredLocalSale(
                    parseCreatedAt = { it?.toLongOrNull() ?: 0L },
                    existingLocalId = existing?.localId,
                    existingOrigin = existing?.origin,
                    existingIdempotencyKey = existing?.idempotencyKey,
                )
                sales[mirrored.localId] = mirrored
            }
            flow.value = sales.values.toList()
        }
    }

    override suspend fun upsertSyncedRemoteSale(sale: com.fullsales.seller.shared.model.Sale) {
        upsertFromRemoteSales(listOf(sale))
    }

    private var lastSalesSync: Long? = null

    override suspend fun getLastSalesSyncEpochMs(): Long? = lastSalesSync

    override suspend fun setLastSalesSyncEpochMs(epochMs: Long) {
        lastSalesSync = epochMs
    }
}

internal class MemoryOutboxRepository : SyncOutboxRepository {
    private val mutex = Mutex()
    private val entries = linkedMapOf<String, SyncOutboxEntry>()

    override suspend fun enqueue(entry: SyncOutboxEntry) = mutex.withLock {
        entries[entry.id] = entry
    }

    override suspend fun listPendingFifo(): List<SyncOutboxEntry> = mutex.withLock {
        entries.values.filter { !it.completed }.sortedBy { it.createdAtEpochMs }
    }

    override suspend fun markCompleted(id: String) {
        mutex.withLock {
            entries[id]?.let { entries[id] = it.copy(completed = true) }
        }
    }

    override suspend fun markFailed(id: String, error: String?) {
        mutex.withLock {
            entries[id]?.let { entries[id] = it.copy(attempts = it.attempts + 1, lastError = error) }
        }
    }

    override suspend fun getEntry(id: String): SyncOutboxEntry? = mutex.withLock {
        entries[id]
    }

    override suspend fun countPendingForAggregate(aggregateId: String): Int = mutex.withLock {
        entries.values.count { !it.completed && it.aggregateId == aggregateId }
    }
}
