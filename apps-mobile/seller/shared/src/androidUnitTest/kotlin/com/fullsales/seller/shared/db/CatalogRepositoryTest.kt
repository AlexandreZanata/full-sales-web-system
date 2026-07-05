package com.fullsales.seller.shared.db

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.fullsales.seller.shared.db.repository.RoomCatalogRepository
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

@RunWith(RobolectricTestRunner::class)
class CatalogRepositoryTest {
    private lateinit var context: Context
    private lateinit var database: SellerDatabase
    private lateinit var catalog: RoomCatalogRepository

    @Before
    fun setUp() {
        context = ApplicationProvider.getApplicationContext()
        database = SellerDatabase.inMemory(context)
        catalog = RoomCatalogRepository(database.catalogDao())
    }

    @After
    fun tearDown() {
        database.close()
    }

    @Test
    fun replaceProducts_clearsStaleProductsAtomically() = runTest {
        catalog.replaceProducts(
            listOf(
                Product("p1", "Alpha", "A-1", 10.0, "BRL", true),
                Product("p2", "Beta", "B-1", 20.0, "BRL", true),
            ),
        )
        catalog.replaceProducts(
            listOf(Product("p3", "Gamma", "G-1", 30.0, "BRL", true)),
        )
        val products = catalog.observeProducts().first()
        assertEquals(1, products.size)
        assertEquals("p3", products.single().id)
    }

    @Test
    fun replaceCommerces_replacesAllRows() = runTest {
        catalog.replaceCommerces(
            listOf(Commerce("c1", "Store A", "A", true)),
        )
        catalog.replaceCommerces(
            listOf(Commerce("c2", "Store B", "B", true)),
        )
        val commerces = catalog.observeCommerces().first()
        assertEquals(1, commerces.size)
        assertEquals("c2", commerces.single().id)
        assertTrue(commerces.single().active)
    }
}
