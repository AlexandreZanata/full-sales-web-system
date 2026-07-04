package com.fullsales.field.shared.sync

import com.fullsales.field.shared.model.Commerce
import com.fullsales.field.shared.model.SaleItem
import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.model.Product
import com.fullsales.field.shared.model.Sale
import com.fullsales.field.shared.model.StockBalance
import com.fullsales.field.shared.model.SyncOutboxEntry
import com.fullsales.field.shared.repository.CatalogRepository
import com.fullsales.field.shared.repository.SaleRepository
import com.fullsales.field.shared.repository.SyncOutboxRepository
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class FakeCatalogRepository : CatalogRepository {
    private val commerces = mutableListOf<Commerce>()
    private val products = mutableListOf<Product>()
    private val stock = mutableMapOf<String, StockBalance>()
    private var lastSync: Long? = null

    override suspend fun listActiveCommerces(): List<Commerce> = commerces.filter { it.active }
    override suspend fun listActiveProducts(): List<Product> = products.filter { it.active }
    override suspend fun getStockBalance(productId: String): StockBalance? = stock[productId]
    override suspend fun replaceCommerces(items: List<Commerce>) {
        commerces.clear()
        commerces.addAll(items)
    }
    override suspend fun replaceProducts(items: List<Product>) {
        products.clear()
        products.addAll(items)
    }
    override suspend fun upsertStockBalance(balance: StockBalance) {
        stock[balance.productId] = balance
    }
    override suspend fun listProductIds(): List<String> = products.map { it.id }
    override suspend fun getLastCatalogSyncEpochMs(): Long? = lastSync
    override suspend fun setLastCatalogSyncEpochMs(epochMs: Long) {
        lastSync = epochMs
    }

    fun seed(product: Product, commerce: Commerce, balance: StockBalance) {
        products.add(product)
        commerces.add(commerce)
        stock[product.id] = balance
    }
}

class FakeSaleRepository : SaleRepository {
    private val mutex = Mutex()
    private val sales = linkedMapOf<String, Sale>()

    override suspend fun listSales(): List<Sale> = mutex.withLock { sales.values.toList() }
    override suspend fun getSale(localId: String): Sale? = mutex.withLock { sales[localId] }
    override suspend fun createOfflineSale(
        localId: String,
        request: CreateSaleRequest,
        totalAmount: Double,
    ): Sale = mutex.withLock {
        val sale = Sale(
            localId = localId,
            commerceId = request.commerceId,
            status = LocalSaleStatus.DraftLocal,
            paymentMethod = request.paymentMethod,
            totalAmount = totalAmount,
            totalCurrency = "BRL",
            items = request.items.map {
                SaleItem(productId = it.productId, quantity = it.quantity)
            },
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

    override suspend fun incrementAttempt(id: String, error: String?) {
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

class FakeTokenRefresher(private var ok: Boolean = true) : SyncTokenRefresher {
    var refreshCount = 0
    override suspend fun refreshToken(): Boolean {
        refreshCount++
        return ok
    }
}
