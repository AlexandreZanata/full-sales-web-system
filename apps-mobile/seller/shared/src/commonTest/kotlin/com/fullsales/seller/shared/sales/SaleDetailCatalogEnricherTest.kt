package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.RecordedMockEngine
import com.fullsales.seller.shared.api.testClient
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.test.runTest

class SaleDetailCatalogEnricherTest {
    @Test
    fun enrich_fetchesMissingCommerceAndProductsWhenOnline() = runTest {
        val commerceId = "01900001-0000-7000-8000-000000000001"
        val productId = "01900001-0000-7000-8000-000000000002"
        val recorder = RecordedMockEngine { request ->
            when (request.url.encodedPath) {
                "/v1/commerces/$commerceId" -> HttpStatusCode.OK to
                    """{"id":"$commerceId","legalName":"Acme LTDA","tradeName":"Acme","active":true}"""
                "/v1/products/$productId" -> HttpStatusCode.OK to
                    """
                    {"id":"$productId","name":"Arroz 5kg","sku":"ARZ-5","priceAmount":899,"priceCurrency":"BRL","active":true,"primaryImageUrl":"/media/x","primaryImageFileId":"file-1"}
                    """.trimIndent()
                else -> HttpStatusCode.NotFound to ""
            }
        }
        val enricher = SaleDetailCatalogEnricher(testClient(engine = recorder.engine()))
        val (commerces, products) = enricher.enrich(
            commerceId = commerceId,
            productIds = listOf(productId),
            commerces = emptyList(),
            products = emptyList(),
            online = true,
        )
        assertEquals("Acme LTDA", commerces.single().legalName)
        assertEquals("Arroz 5kg", products.single().name)
        assertEquals("file-1", products.single().primaryImageFileId)
    }

    @Test
    fun enrich_skipsNetworkWhenOffline() = runTest {
        val enricher = SaleDetailCatalogEnricher(
            testClient(engine = RecordedMockEngine { HttpStatusCode.NotFound to "" }.engine()),
        )
        val (commerces, products) = enricher.enrich(
            commerceId = "c1",
            productIds = listOf("p1"),
            commerces = emptyList(),
            products = emptyList(),
            online = false,
        )
        assertEquals(emptyList<Commerce>(), commerces)
        assertEquals(emptyList<Product>(), products)
    }
}
