package com.fullsales.seller.shared.db

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.fullsales.seller.shared.db.repository.RoomSaleRepository
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SaleItem
import com.fullsales.seller.shared.model.SaleOrigin
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

/**
 * T-16-20 — remote-mirrored sale + lines survive Room reopen.
 */
@RunWith(RobolectricTestRunner::class)
class RemoteSalePersistenceTest {
    private lateinit var context: Context

    @Before
    fun setUp() {
        context = ApplicationProvider.getApplicationContext()
        context.deleteDatabase(DB_NAME)
    }

    @After
    fun tearDown() {
        context.deleteDatabase(DB_NAME)
    }

    @Test
    fun given_mirroredRemoteSale_when_reopenDb_then_saleAndLinesSurvive() = runTest {
        val db1 = SellerDatabase.build(context, DB_NAME)
        val repo1 = RoomSaleRepository(db1.saleDao(), db1.catalogDao())
        repo1.upsertFromRemoteSales(
            listOf(
                Sale(
                    id = "remote-persist-1",
                    commerceId = "c1",
                    driverId = "driver-1",
                    status = "Confirmed",
                    paymentMethod = "debit",
                    totalAmount = 40.0,
                    totalCurrency = "BRL",
                    items = listOf(SaleItem("p1", 2, 20.0, "BRL", 40.0)),
                    createdAt = null,
                ),
            ),
        )
        repo1.setLastSalesSyncEpochMs(99L)
        db1.close()

        val db2 = SellerDatabase.build(context, DB_NAME)
        val repo2 = RoomSaleRepository(db2.saleDao(), db2.catalogDao())
        val restored = repo2.getSale("remote-persist-1")
        val syncAt = repo2.getLastSalesSyncEpochMs()
        val count = repo2.observeSales().first().size
        db2.close()

        assertNotNull(restored)
        assertEquals(LocalSaleStatus.Confirmed, restored!!.status)
        assertEquals(SaleOrigin.RemoteMirror, restored.origin)
        assertEquals("driver-1", restored.driverId)
        assertEquals(1, restored.items.size)
        assertEquals(20.0, restored.items.single().unitPriceAmount, 0.001)
        assertEquals(40.0, restored.items.single().lineTotalAmount, 0.001)
        assertEquals(99L, syncAt)
        assertEquals(1, count)
    }

    private companion object {
        const val DB_NAME = "seller-remote-sale-persist.db"
    }
}
