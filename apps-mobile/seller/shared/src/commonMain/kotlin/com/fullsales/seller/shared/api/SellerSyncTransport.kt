package com.fullsales.seller.shared.api

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.sync.CatalogPullClient
import com.fullsales.seller.shared.sync.SyncHttpOutcome
import com.fullsales.seller.shared.sync.SyncHttpResult
import com.fullsales.seller.shared.sync.SyncTransport
import kotlinx.serialization.json.Json

class SellerSyncTransport(
    private val client: SellerApiClient,
    private val json: Json = defaultSellerJson(),
) : SyncTransport, CatalogPullClient {
    override suspend fun fetchCommerces(page: Int, pageSize: Int): List<Commerce> =
        client.listCommerces(page, pageSize).items

    override suspend fun fetchProducts(limit: Int, cursor: String?): com.fullsales.seller.shared.model.CursorListProducts =
        client.listProducts(limit, cursor)

    override suspend fun execute(entry: SyncOutboxEntry): SyncHttpResult = try {
        when (entry.method) {
            "POST" -> postOutbox(entry)
            else -> SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = "UNSUPPORTED_METHOD")
        }
    } catch (error: ApiException) {
        mapApiError(error)
    } catch (error: Exception) {
        SyncHttpResult(SyncHttpOutcome.NetworkError, errorCode = error.message)
    }

    private suspend fun postOutbox(entry: SyncOutboxEntry): SyncHttpResult = when {
        entry.path == "/sales" -> createSale(entry)
        entry.path.endsWith("/confirm") -> postSaleAction(entry) { id -> client.confirmSale(id) }
        entry.path.endsWith("/cancel") -> postSaleAction(entry) { id -> client.cancelSale(id) }
        else -> SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = "UNKNOWN_PATH")
    }

    private suspend fun createSale(entry: SyncOutboxEntry): SyncHttpResult {
        val request = json.decodeFromString<CreateSaleRequest>(entry.bodyJson)
        val sale = client.createSale(request, entry.idempotencyKey)
        return SyncHttpResult(SyncHttpOutcome.Success, remoteId = sale.id)
    }

    private suspend fun postSaleAction(
        entry: SyncOutboxEntry,
        action: suspend (String) -> com.fullsales.seller.shared.model.Sale,
    ): SyncHttpResult {
        val saleId = saleIdFromPath(entry.path)
            ?: return SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = "INVALID_PATH")
        action(saleId)
        return SyncHttpResult(SyncHttpOutcome.Success, remoteId = saleId)
    }

    private fun mapApiError(error: ApiException): SyncHttpResult {
        val code = error.detail.code
        return when {
            error.statusCode == 401 ->
                SyncHttpResult(SyncHttpOutcome.Unauthorized, errorCode = code)
            error.statusCode == 409 && code.equals("INSUFFICIENT_STOCK", ignoreCase = true) ->
                SyncHttpResult(SyncHttpOutcome.InsufficientStock, errorCode = code)
            code.equals("INSUFFICIENT_STOCK", ignoreCase = true) ->
                SyncHttpResult(SyncHttpOutcome.InsufficientStock, errorCode = code)
            else -> SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = code)
        }
    }

    private fun saleIdFromPath(path: String): String? {
        val parts = path.trim('/').split('/')
        if (parts.size >= 2 && parts[0] == "sales") return parts[1]
        return null
    }
}
