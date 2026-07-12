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
        products.clear()
        products.addAll(items)
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

    override suspend fun countPendingForSale(saleLocalId: String): Int = mutex.withLock {
        entries.values.count { !it.completed && it.saleLocalId == saleLocalId }
    }
}
