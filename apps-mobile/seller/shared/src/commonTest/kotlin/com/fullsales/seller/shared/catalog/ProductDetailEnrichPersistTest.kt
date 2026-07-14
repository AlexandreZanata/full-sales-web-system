package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.api.RecordedMockEngine
import com.fullsales.seller.shared.api.testClient
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.repository.MemoryStockSnapshotRepository
import com.fullsales.seller.shared.sync.FakeCatalogRepository
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.test.runTest

/**
 * T-16-01 (common) — online detail enrich persists UOM/description into catalog repo.
 */
class ProductDetailEnrichPersistTest {
    @Test
    fun given_onlineDetail_when_load_then_catalogStoresUomAndDescription() = runTest {
        val catalog = FakeCatalogRepository()
        catalog.seed(
            Product("p1", "Widget", "W-1", 10.0, "BRL", true),
            Commerce("c1", "Acme", "Acme", true),
        )
        val recorder = RecordedMockEngine { request ->
            when {
                request.url.encodedPath.endsWith("/products/p1") ->
                    HttpStatusCode.OK to """
                        {"id":"p1","name":"Widget","sku":"W-1","priceAmount":10.0,
                        "priceCurrency":"BRL","active":true,"unitOfMeasure":"cx",
                        "description":"Box of widgets"}
                    """.trimIndent()
                request.url.encodedPath.contains("/balance") ->
                    HttpStatusCode.OK to """{"productId":"p1","available":3}"""
                else -> error("unexpected ${request.url}")
            }
        }
        val loader = ProductDetailLoader(
            catalog,
            MemoryStockSnapshotRepository(),
            testClient(engine = recorder.engine()),
        )
        val result = loader.load("p1", online = true)
        assertEquals("cx", result.product.unitOfMeasure)
        assertEquals("Box of widgets", catalog.getProduct("p1")!!.description)
        assertEquals("cx", catalog.getProduct("p1")!!.unitOfMeasure)
    }
}
