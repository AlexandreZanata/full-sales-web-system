package com.fullsales.seller.shared.db

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.fullsales.seller.shared.catalog.toDetail
import com.fullsales.seller.shared.db.repository.RoomCatalogRepository
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

/**
 * T-16-01 / T-16-02 — catalog field round-trip in Room LocalStore.
 */
@RunWith(RobolectricTestRunner::class)
class CatalogFieldPersistenceTest {
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
    fun given_productWithDescriptionAndUom_when_upsertAndGet_then_fieldsRoundTrip() = runTest {
        catalog.upsertProducts(
            listOf(
                Product(
                    id = "p1",
                    name = "Widget",
                    sku = "W-1",
                    priceAmount = 10.0,
                    priceCurrency = "BRL",
                    active = true,
                    unitOfMeasure = "kg",
                    description = "Bulk widget",
                ),
            ),
        )
        val stored = catalog.getProduct("p1")!!
        assertEquals("kg", stored.unitOfMeasure)
        assertEquals("Bulk widget", stored.description)
        assertEquals("kg", stored.toDetail().unitOfMeasure)
        assertEquals("Bulk widget", stored.toDetail().description)
    }

    @Test
    fun given_commerceWithCnpj_when_replaceAndObserve_then_cnpjPresent() = runTest {
        catalog.replaceCommerces(
            listOf(
                Commerce(
                    id = "c1",
                    legalName = "Acme Ltda",
                    tradeName = "Acme",
                    active = true,
                    cnpj = "12345678000199",
                ),
            ),
        )
        val commerce = catalog.observeCommerces().first().single()
        assertEquals("12345678000199", commerce.cnpj)
        assertEquals("12345678000199", catalog.getCommerce("c1")!!.cnpj)
    }

    @Test
    fun given_detailFields_when_listPullOmitsThem_then_uomAndDescriptionPreserved() = runTest {
        catalog.upsertProducts(
            listOf(
                Product(
                    id = "p1",
                    name = "Widget",
                    sku = "W-1",
                    priceAmount = 10.0,
                    priceCurrency = "BRL",
                    active = true,
                    unitOfMeasure = "un",
                    description = "Keep me",
                ),
            ),
        )
        catalog.replaceProducts(
            listOf(
                Product(
                    id = "p1",
                    name = "Widget Renamed",
                    sku = "W-1",
                    priceAmount = 11.0,
                    priceCurrency = "BRL",
                    active = true,
                ),
            ),
        )
        val stored = catalog.getProduct("p1")!!
        assertEquals("Widget Renamed", stored.name)
        assertEquals("un", stored.unitOfMeasure)
        assertEquals("Keep me", stored.description)
    }
}
