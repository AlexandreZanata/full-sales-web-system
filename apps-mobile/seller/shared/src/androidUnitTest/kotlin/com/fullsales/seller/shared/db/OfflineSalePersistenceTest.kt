package com.fullsales.seller.shared.db

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.fullsales.seller.shared.db.repository.RoomSaleRepository
import com.fullsales.seller.shared.db.repository.RoomSyncOutboxRepository
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

@RunWith(RobolectricTestRunner::class)
class OfflineSalePersistenceTest {
    private lateinit var context: Context
    private lateinit var database: SellerDatabase
    private lateinit var saleRepository: RoomSaleRepository
    private lateinit var outboxRepository: RoomSyncOutboxRepository
    private lateinit var writer: OfflineSaleWriter

    @Before
    fun setUp() {
        context = ApplicationProvider.getApplicationContext()
        database = SellerDatabase.inMemory(context)
        saleRepository = RoomSaleRepository(database.saleDao(), database.catalogDao())
        outboxRepository = RoomSyncOutboxRepository(database.syncOutboxDao())
        writer = OfflineSaleWriter(saleRepository, outboxRepository)
    }

    @After
    fun tearDown() {
        database.close()
    }

    @Test
    fun createOfflineSale_appearsInObserveSales() = runTest {
        writer.createSale(saleRequest(), totalAmount = 20.0)
        val sales = saleRepository.observeSales().first()
        assertEquals(1, sales.size)
        assertEquals(LocalSaleStatus.PendingSync, sales.single().status)
        assertNotNull(sales.single().idempotencyKey)
    }

    @Test
    fun createOfflineSale_enqueuesOutboxRow() = runTest {
        val sale = writer.createSale(saleRequest(), totalAmount = 20.0)
        assertTrue(outboxRepository.countPendingForSale(sale.localId) >= 1)
        val pending = outboxRepository.listPendingFifo()
        assertEquals(sale.idempotencyKey, pending.single().idempotencyKey)
    }

    @Test
    fun offlineSale_survivesDatabaseReopen() = runTest {
        val dbName = "seller-persistence-test.db"
        context.deleteDatabase(dbName)
        val db1 = SellerDatabase.build(context, dbName)
        val sales1 = RoomSaleRepository(db1.saleDao(), db1.catalogDao())
        val outbox1 = RoomSyncOutboxRepository(db1.syncOutboxDao())
        val localId = OfflineSaleWriter(sales1, outbox1)
            .createSale(saleRequest(), totalAmount = 15.0)
            .localId
        db1.close()

        val db2 = SellerDatabase.build(context, dbName)
        val restored = RoomSaleRepository(db2.saleDao(), db2.catalogDao()).getSale(localId)
        db2.close()
        context.deleteDatabase(dbName)

        assertNotNull(restored)
        assertEquals(LocalSaleStatus.PendingSync, restored!!.status)
    }

    private fun saleRequest() = CreateSaleRequest(
        commerceId = "commerce-1",
        paymentMethod = "cash",
        items = listOf(CreateSaleItem("product-1", 2)),
    )
}
