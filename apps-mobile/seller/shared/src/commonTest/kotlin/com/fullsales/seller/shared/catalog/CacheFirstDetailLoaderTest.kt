package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.MemoryCommerceAddressCache
import com.fullsales.seller.shared.repository.MemoryStockSnapshotRepository
import com.fullsales.seller.shared.sync.FakeCatalogRepository
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

/**
 * Contract (Phase 14D): offline product/commerce detail from cache after prior sync.
 */
class CacheFirstDetailLoaderTest {
    @Test
    fun given_cachedProduct_when_offline_then_returnsRoomProductAndStock() = runTest {
        val catalog = FakeCatalogRepository()
        catalog.seed(
            Product("p1", "Widget", "W-1", 10.0, "BRL", true),
            Commerce("c1", "Acme", "Acme", true),
        )
        val stock = MemoryStockSnapshotRepository()
        stock.put("p1", 7)
        val loader = ProductDetailLoader(catalog, stock, unusedApi())
        val result = loader.load("p1", online = false)
        assertEquals("Widget", result.product.name)
        assertEquals(7, result.stockAvailable)
        assertTrue(result.fromCache)
    }

    @Test
    fun given_emptyCache_when_offline_then_fails() = runTest {
        val loader = ProductDetailLoader(
            FakeCatalogRepository(),
            MemoryStockSnapshotRepository(),
            unusedApi(),
        )
        assertFailsWith<IllegalStateException> { loader.load("missing", online = false) }
    }

    @Test
    fun given_cachedCommerce_when_offline_then_returnsCachedAddresses() = runTest {
        val catalog = FakeCatalogRepository()
        catalog.seed(
            Product("p1", "W", "W", 1.0, "BRL", true),
            Commerce("c1", "Acme Ltd", "Acme", true),
        )
        val addresses = MemoryCommerceAddressCache()
        addresses.put(
            "c1",
            listOf(
                com.fullsales.seller.shared.model.CommerceAddress(
                    id = "a1",
                    type = "Delivery",
                    street = "Main",
                    number = "1",
                    city = "SP",
                    state = "SP",
                    postalCode = "01000",
                    isPrimary = true,
                ),
            ),
        )
        val loader = CommerceDetailLoader(catalog, addresses, unusedApi())
        val result = loader.load("c1", online = false)
        assertEquals("Acme Ltd", result.commerce.legalName)
        assertEquals(1, result.addresses.size)
        assertTrue(result.fromCache)
    }

    private fun unusedApi() = com.fullsales.seller.shared.api.SellerApiClient(
        io.ktor.client.HttpClient(io.ktor.client.engine.mock.MockEngine { error("offline") }),
        baseUrl = "http://test/v1",
    )
}
