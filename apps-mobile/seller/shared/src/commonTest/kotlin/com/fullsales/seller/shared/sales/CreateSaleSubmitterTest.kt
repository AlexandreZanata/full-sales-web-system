package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.i18n.CreateSaleValidationError
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
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
    fun validateForm_blocksQuantityAboveStock() {
        val errors = validateCreateSaleForm(
            commerceId = "c1",
            paymentMethod = "cash",
            lines = listOf(CreateSaleLineInput("p1", "5")),
            stockByProductId = mapOf("p1" to 2),
        )
        assertTrue(!errors.isValid)
        assertEquals(
            CreateSaleValidationError.QuantityExceedsStock(2),
            errors.lineErrors.first().quantityError,
        )
    }
}
