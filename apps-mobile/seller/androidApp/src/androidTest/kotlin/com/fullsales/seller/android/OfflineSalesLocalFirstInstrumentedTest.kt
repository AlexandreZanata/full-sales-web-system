package com.fullsales.seller.android

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleDisplayStatus
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import com.fullsales.seller.shared.sales.SaleDetailLoader
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

/**
 * T-16-30 / T-16-32 — LocalStore sales remain readable offline after seed (airplane path).
 */
@RunWith(AndroidJUnit4::class)
class OfflineSalesLocalFirstInstrumentedTest {
    private lateinit var container: AppContainer

    @Before
    fun setUp() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        context.deleteDatabase("seller.db")
        container = AppContainer(context)
    }

    @Test
    fun given_seededMirroredSale_when_offlineObserve_then_listHasRow() = runBlocking {
        container.saleRepository.upsertFromRemoteSales(listOf(sampleRemoteSale()))
        container.saleRepository.setLastSalesSyncEpochMs(1L)
        val rows = container.saleRepository.observeSales().first()
        assertEquals(1, rows.size)
        assertEquals("remote-air-1", rows.single().localId)
        assertEquals(SaleOrigin.RemoteMirror, rows.single().origin)
    }

    @Test
    fun given_seededMirroredSale_when_offlineDetail_then_resolvesFromLocalStore() = runBlocking {
        container.saleRepository.upsertFromRemoteSales(listOf(sampleRemoteSale()))
        val loader = SaleDetailLoader(
            container.apiClient,
            container.saleRepository,
            container.outboxRepository,
        )
        val result = loader.load(
            id = "remote-air-1",
            commerces = emptyList(),
            products = emptyList(),
            online = false,
        )
        assertTrue(result.isSuccess)
        val detail = result.getOrThrow()
        assertEquals("remote-air-1", detail.navigationId)
        assertEquals(SaleDisplayStatus.Confirmed, detail.status)
    }

    private fun sampleRemoteSale() = Sale(
        id = "remote-air-1",
        commerceId = "c1",
        driverId = "d1",
        status = "Confirmed",
        paymentMethod = "cash",
        totalAmount = 10.0,
        totalCurrency = "BRL",
        items = listOf(SaleItem("p1", 1, 10.0, "BRL", 10.0)),
        createdAt = null,
    )
}
