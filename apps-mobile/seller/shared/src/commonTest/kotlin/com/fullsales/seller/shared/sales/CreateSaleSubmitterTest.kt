package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.sync.FakeOutboxRepository
import com.fullsales.seller.shared.sync.FakeSaleRepository
import com.fullsales.seller.shared.sync.OfflineSaleWriter
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest

class CreateSaleSubmitterTest {
    private val sales = FakeSaleRepository()
    private val outbox = FakeOutboxRepository()
    private val writer = OfflineSaleWriter(sales, outbox)
    private val unusedApi = SellerApiClient(
        HttpClient(MockEngine { error("online path not used") }),
        baseUrl = "http://test/v1",
    )

    @Test
    fun offlineSubmit_createsOutboxEntry() = runTest {
        val submitter = CreateSaleSubmitter(apiClient = unusedApi, offlineWriter = writer)
        val request = CreateSaleRequest(
            commerceId = "c1",
            paymentMethod = "cash",
            items = listOf(CreateSaleItem("p1", 2)),
        )
        val result = submitter.submit(request, totalAmountMinor = 2000.0, online = false)
        assertIs<CreateSaleSubmitResult.Success>(result)
        assertEquals(false, result.isRemote)
        assertEquals(1, outbox.all.count { !it.completed })
        assertEquals(LocalSaleStatus.PendingSync, sales.getSale(result.navigationId)?.status)
    }

    @Test
    fun given_onlineTransportFailure_when_submit_then_fallsBackToOutbox() = runTest {
        val failingApi = SellerApiClient(
            HttpClient(MockEngine { error("connection timeout") }),
            baseUrl = "http://test/v1",
        )
        val submitter = CreateSaleSubmitter(apiClient = failingApi, offlineWriter = writer)
        val request = CreateSaleRequest(
            commerceId = "c1",
            paymentMethod = "cash",
            items = listOf(CreateSaleItem("p1", 1)),
        )
        val result = submitter.submit(request, totalAmountMinor = 1000.0, online = true)
        assertIs<CreateSaleSubmitResult.Success>(result)
        assertEquals(false, result.isRemote)
        assertEquals(1, outbox.all.count { !it.completed })
        assertEquals(LocalSaleStatus.PendingSync, sales.getSale(result.navigationId)?.status)
    }

    @Test
    fun validateForm_allowsQuantityAboveStock() {
        val errors = validateCreateSaleForm(
            commerceId = "c1",
            paymentMethod = "cash",
            lines = listOf(CreateSaleLineInput("p1", "5")),
            stockByProductId = mapOf("p1" to 2),
        )
        assertTrue(errors.isValid)
    }

    @Test
    fun needsBackorderWarning_whenQuantityExceedsStock() {
        assertTrue(
            needsBackorderWarning(
                productId = "p1",
                lines = listOf(CreateSaleLineInput("p1", "5")),
                stockByProductId = mapOf("p1" to 2),
            ),
        )
    }

    @Test
    fun saleLineNeedsBackorderWarning_whenQuantityExceedsStock() {
        assertTrue(
            saleLineNeedsBackorderWarning(
                productId = "p1",
                quantity = 11,
                stockByProductId = mapOf("p1" to 2),
            ),
        )
    }

    @Test
    fun filterProductsAvailableForBrowsing_hidesZeroStock() {
        val products = listOf(
            Product(
                id = "p1",
                sku = "A",
                name = "A",
                priceAmount = 100.0,
                priceCurrency = "BRL",
                active = true,
            ),
            Product(
                id = "p2",
                sku = "B",
                name = "B",
                priceAmount = 100.0,
                priceCurrency = "BRL",
                active = true,
            ),
        )
        val filtered = filterProductsAvailableForBrowsing(products, mapOf("p1" to 0, "p2" to 3))
        assertEquals(listOf("p2"), filtered.map { it.id })
    }
}
