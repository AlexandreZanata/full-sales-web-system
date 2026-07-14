package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CreateSaleItem
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
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class FakeCatalogRepository : CatalogRepository {
    private val commerces = mutableListOf<Commerce>()
    private val products = mutableListOf<Product>()
    private var lastSync: Long? = null

    override fun observeCommerces(): Flow<List<Commerce>> =
        flowOf(commerces.filter { it.active })

    override fun observeProducts(): Flow<List<Product>> =
        flowOf(products.filter { it.active })

    override suspend fun getCommerce(id: String): Commerce? = commerces.firstOrNull { it.id == id }

    override suspend fun getProduct(id: String): Product? = products.firstOrNull { it.id == id }

    override suspend fun replaceCommerces(items: List<Commerce>) {
        commerces.clear()
        commerces.addAll(items)
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
    }

    override suspend fun upsertProducts(items: List<Product>) {
        items.forEach { incoming ->
            val idx = products.indexOfFirst { it.id == incoming.id }
            if (idx >= 0) products[idx] = incoming else products.add(incoming)
        }
    }

    override suspend fun getLastCatalogSyncEpochMs(): Long? = lastSync

    override suspend fun setLastCatalogSyncEpochMs(epochMs: Long) {
        lastSync = epochMs
    }

    fun seed(product: Product, commerce: Commerce) {
        products.add(product)
        commerces.add(commerce)
    }
}

class FakeSaleRepository : SaleRepository {
    private val mutex = Mutex()
    private val sales = linkedMapOf<String, LocalSale>()

    override fun observeSales(): Flow<List<LocalSale>> = flowOf(sales.values.toList())

    override suspend fun getSale(localId: String): LocalSale? = mutex.withLock { sales[localId] }

    override suspend fun getSaleByRemoteId(remoteId: String): LocalSale? = mutex.withLock {
        sales.values.firstOrNull { it.remoteId == remoteId }
    }

    override suspend fun createLocalSale(
        request: CreateSaleRequest,
        totalAmount: Double,
    ): LocalSale = mutex.withLock {
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
        sale
    }

    override suspend fun updateStatus(localId: String, status: LocalSaleStatus) {
        mutex.withLock {
            sales[localId]?.let { sales[localId] = it.copy(status = status) }
        }
    }

    override suspend fun setRemoteId(localId: String, remoteId: String, status: LocalSaleStatus) {
        mutex.withLock {
            sales[localId]?.let { sales[localId] = it.copy(remoteId = remoteId, status = status) }
        }
    }

    override suspend fun markSyncFailed(localId: String, reason: String) {
        mutex.withLock {
            sales[localId]?.let {
                sales[localId] = it.copy(status = LocalSaleStatus.SyncFailed, syncFailureReason = reason)
            }
        }
    }

    override suspend fun upsertFromRemoteSales(remoteSales: List<com.fullsales.seller.shared.model.Sale>) {
        mutex.withLock {
            remoteSales.forEach { remote ->
                val existing = sales.values.firstOrNull { it.remoteId == remote.id }
                    ?: sales[remote.id]
                val mirrored = remote.toMirroredLocalSale(
                    parseCreatedAt = { it?.toLongOrNull() ?: 0L },
                    existingLocalId = existing?.localId,
                    existingOrigin = existing?.origin,
                    existingIdempotencyKey = existing?.idempotencyKey,
                )
                sales[mirrored.localId] = mirrored
            }
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

class FakeOutboxRepository : SyncOutboxRepository {
    private val mutex = Mutex()
    private val entries = linkedMapOf<String, SyncOutboxEntry>()

    val all: List<SyncOutboxEntry> get() = entries.values.toList()

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

class RecordingTransport : SyncTransport {
    val calls = mutableListOf<SyncOutboxEntry>()
    var nextResult: SyncHttpResult = SyncHttpResult(SyncHttpOutcome.Success, remoteId = "remote-1")
    private val results = mutableMapOf<String, SyncHttpResult>()

    fun stub(entryId: String, result: SyncHttpResult) {
        results[entryId] = result
    }

    override suspend fun execute(entry: SyncOutboxEntry): SyncHttpResult {
        calls.add(entry)
        return results[entry.id] ?: nextResult
    }
}

class FakeCatalogPullClient : CatalogPullClient, SalesPullClient, RegistrationsPullClient {
    var commerces = listOf<Commerce>()
    var products = listOf<Product>()
    var sales = listOf<com.fullsales.seller.shared.model.Sale>()
    var registrations = listOf<com.fullsales.seller.shared.model.CommerceRegistration>()
    var throwOnFetch: Boolean = false
    var throwOnSalesFetch: Boolean = false
    var throwOnRegistrationsFetch: Boolean = false

    override suspend fun fetchCommerces(limit: Int, cursor: String?): com.fullsales.seller.shared.model.CursorListCommerces {
        if (throwOnFetch) error("catalog unavailable")
        return if (cursor == null) {
            com.fullsales.seller.shared.model.CursorListCommerces(
                commerces,
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        } else {
            com.fullsales.seller.shared.model.CursorListCommerces(
                emptyList(),
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        }
    }

    override suspend fun fetchProducts(limit: Int, cursor: String?): com.fullsales.seller.shared.model.CursorListProducts {
        if (throwOnFetch) error("catalog unavailable")
        return if (cursor == null) {
            com.fullsales.seller.shared.model.CursorListProducts(
                products,
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        } else {
            com.fullsales.seller.shared.model.CursorListProducts(
                emptyList(),
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        }
    }

    override suspend fun fetchSales(limit: Int, cursor: String?): com.fullsales.seller.shared.model.CursorListSales {
        if (throwOnSalesFetch) error("sales unavailable")
        return if (cursor == null) {
            com.fullsales.seller.shared.model.CursorListSales(
                sales,
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        } else {
            com.fullsales.seller.shared.model.CursorListSales(
                emptyList(),
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        }
    }

    override suspend fun fetchRegistrations(limit: Int, cursor: String?): com.fullsales.seller.shared.model.CursorListRegistrations {
        if (throwOnRegistrationsFetch) error("registrations unavailable")
        return if (cursor == null) {
            com.fullsales.seller.shared.model.CursorListRegistrations(
                registrations,
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        } else {
            com.fullsales.seller.shared.model.CursorListRegistrations(
                emptyList(),
                com.fullsales.seller.shared.model.CursorPaginationMeta(null, false, limit),
            )
        }
    }
}

class FakeTokenRefresher(private var ok: Boolean = true) : SyncTokenRefresher {
    var refreshCount = 0
    override suspend fun refreshToken(): Boolean {
        refreshCount++
        return ok
    }
}
