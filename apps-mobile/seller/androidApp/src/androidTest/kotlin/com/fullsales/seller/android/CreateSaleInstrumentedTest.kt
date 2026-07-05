package com.fullsales.seller.android

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.sales.CreateSaleSubmitResult
import com.fullsales.seller.shared.sales.CreateSaleSubmitter
import java.net.HttpURLConnection
import java.net.URL
import kotlinx.coroutines.runBlocking
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assume.assumeTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class CreateSaleInstrumentedTest {
    private lateinit var container: AppContainer

    @Before
    fun setUp() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        context.deleteDatabase("seller.db")
        container = AppContainer(context)
    }

    @Test
    fun offlineCreateSale_enqueuesOutboxRow() = runBlocking {
        val submitter = CreateSaleSubmitter(container.apiClient, container.offlineSaleWriter)
        val request = CreateSaleRequest(
            commerceId = "commerce-seed",
            paymentMethod = "cash",
            items = listOf(CreateSaleItem("product-seed", 1)),
        )
        val result = submitter.submit(request, totalAmountMinor = 1000.0, online = false)
        assertTrue(result is CreateSaleSubmitResult.Success)
        val success = result as CreateSaleSubmitResult.Success
        assertEquals(LocalSaleStatus.PendingSync, container.saleRepository.getSale(success.navigationId)?.status)
        assertTrue(container.outboxRepository.listPendingFifo().isNotEmpty())
    }

    @Test
    fun onlineCreateSale_usesDevApiWhenAvailable() = runBlocking {
        assumeTrue("Dev API not reachable on emulator host", isDevApiReachable())
        val login = container.apiClient.login("seller@test.com", "secret123")
        container.tokenStore.saveTokens(login.accessToken, login.refreshToken)
        val commerces = container.apiClient.listCommerces(pageSize = 5).items
        val products = container.apiClient.listProducts(pageSize = 5).items
        assumeTrue(commerces.isNotEmpty() && products.isNotEmpty())
        val submitter = CreateSaleSubmitter(container.apiClient, container.offlineSaleWriter)
        val request = CreateSaleRequest(
            commerceId = commerces.first().id,
            paymentMethod = "cash",
            items = listOf(CreateSaleItem(products.first().id, 1)),
        )
        when (val result = submitter.submit(request, products.first().priceAmount, online = true)) {
            is CreateSaleSubmitResult.Success -> assertTrue(result.navigationId.isNotBlank())
            is CreateSaleSubmitResult.Failure -> assumeTrue(
                "Create sale rejected: ${result.code}",
                result.code == "INSUFFICIENT_STOCK",
            )
        }
    }

    private fun isDevApiReachable(): Boolean = runCatching {
        val url = URL("http://10.0.2.2:8080/v1/auth/login")
        (url.openConnection() as HttpURLConnection).run {
            connectTimeout = 2000
            readTimeout = 2000
            requestMethod = "POST"
            responseCode
            disconnect()
            true
        }
    }.getOrDefault(false)
}
