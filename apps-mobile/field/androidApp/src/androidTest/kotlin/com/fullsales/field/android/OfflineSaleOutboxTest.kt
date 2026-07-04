package com.fullsales.field.android

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.fullsales.field.shared.model.CreateSaleItem
import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.LocalSaleStatus
import com.fullsales.field.shared.sync.OfflineSaleWriter
import java.util.UUID
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
        context.deleteDatabase("field.db")
        container = AppContainer(context)
    }

    @Test
    fun createSaleOffline_enqueuesOutboxRow() = runBlocking {
        val localId = UUID.randomUUID().toString()
        val writer = OfflineSaleWriter(container.saleRepository, container.outboxRepository)
        writer.createSale(
            localId,
            CreateSaleRequest(
                commerceId = "commerce-1",
                paymentMethod = "cash",
                items = listOf(CreateSaleItem("product-1", 2)),
            ),
            totalAmount = 20.0,
        )

        val sale = container.saleRepository.getSale(localId)
        assertEquals(LocalSaleStatus.PendingSync, sale?.status)
        assertTrue(container.outboxRepository.countPendingForSale(localId) >= 1)
    }
}
