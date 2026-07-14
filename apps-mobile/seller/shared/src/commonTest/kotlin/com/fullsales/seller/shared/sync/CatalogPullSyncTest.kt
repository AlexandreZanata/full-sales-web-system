package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest

class CatalogPullSyncTest {
    @Test
    fun pullCatalog_replacesCatalogAndRecordsLastSync() = runTest {
        val catalog = FakeCatalogRepository()
        val client = FakeCatalogPullClient()
        client.commerces = listOf(Commerce("c1", "Store", "S", true, cnpj = "11222333000181"))
        client.products = listOf(
            Product("p1", "Active", "A-1", 10.0, "BRL", true),
            Product("p2", "Inactive", "I-1", 5.0, "BRL", false),
        )
        val sync = CatalogPullSync(catalog, client)

        sync.pullCatalog(nowEpochMs = 99_000L)

        assertEquals(1, catalog.observeCommerces().first().size)
        assertEquals("11222333000181", catalog.getCommerce("c1")!!.cnpj)
        val products = catalog.observeProducts().first()
        assertEquals(1, products.size)
        assertEquals("p1", products.single().id)
        assertEquals(99_000L, catalog.getLastCatalogSyncEpochMs())
    }
}
