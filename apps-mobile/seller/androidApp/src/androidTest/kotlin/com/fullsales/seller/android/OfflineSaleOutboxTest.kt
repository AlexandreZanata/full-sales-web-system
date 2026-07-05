package com.fullsales.seller.android

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import kotlinx.coroutines.runBlocking
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class OfflineSaleOutboxTest {
    private lateinit var container: AppContainer

    @Before
    fun setUp() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        context.deleteDatabase("seller.db")
        container = AppContainer(context)
    }

    @Test
    fun createSaleOffline_enqueuesOutboxRow() = runBlocking {
        val writer = OfflineSaleWriter(container.saleRepository, container.outboxRepository)
        val sale = writer.createSale(
            CreateSaleRequest(
                commerceId = "commerce-1",
                paymentMethod = "cash",
                items = listOf(CreateSaleItem("product-1", 2)),
            ),
            totalAmount = 20.0,
        )

        assertEquals(LocalSaleStatus.PendingSync, container.saleRepository.getSale(sale.localId)?.status)
        assertTrue(container.outboxRepository.countPendingForSale(sale.localId) >= 1)
    }
}
