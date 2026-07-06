package com.fullsales.field.shared.api

import com.fullsales.field.shared.model.CreateSaleRequest
import com.fullsales.field.shared.model.CursorListCommerces
import com.fullsales.field.shared.model.PaginatedProducts
import com.fullsales.field.shared.model.SaleDto
import com.fullsales.field.shared.model.StockBalance
import com.fullsales.field.shared.model.SyncOutboxEntry
import com.fullsales.field.shared.sync.CatalogPullClient
import com.fullsales.field.shared.sync.SyncHttpOutcome
import com.fullsales.field.shared.sync.SyncHttpResult
import com.fullsales.field.shared.sync.SyncTransport
import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.request.get
import io.ktor.client.request.header
import io.ktor.client.request.post
import io.ktor.client.request.setBody
import io.ktor.client.statement.bodyAsText
import io.ktor.http.ContentType
import io.ktor.http.HttpStatusCode
import io.ktor.http.contentType
import kotlinx.serialization.json.Json

class FieldApiClient(
    private val http: HttpClient,
    private val tokenProvider: AuthTokenProvider,
    private val json: Json = Json { ignoreUnknownKeys = true },
) : CatalogPullClient, SyncTransport {
    override suspend fun fetchCommerces(limit: Int, cursor: String?): CursorListCommerces {
        val cursorParam = cursor?.let { "&cursor=$it" } ?: ""
        return authorizedGet("/commerces?limit=$limit$cursorParam")
    }

    override suspend fun fetchProducts(page: Int, pageSize: Int) =
        getPaginated<PaginatedProducts>("/products?page=$page&pageSize=$pageSize").items

    override suspend fun fetchStockBalance(productId: String): StockBalance? = runCatching {
        authorizedGet<StockBalance>("/inventory/products/$productId/balance")
    }.getOrNull()

    override suspend fun execute(entry: SyncOutboxEntry): SyncHttpResult = runCatching {
        when (entry.method) {
            "POST" -> postOutbox(entry)
            else -> SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = "UNSUPPORTED_METHOD")
        }
    }.getOrElse {
        SyncHttpResult(SyncHttpOutcome.NetworkError, errorCode = it.message)
    }

    private suspend fun postOutbox(entry: SyncOutboxEntry): SyncHttpResult {
        val response = http.post("$FIELD_API_BASE_URL${entry.path}") {
            contentType(ContentType.Application.Json)
            tokenProvider.accessToken()?.let { header("Authorization", "Bearer $it") }
            header("Idempotency-Key", entry.idempotencyKey)
            if (entry.path == "/sales") {
                setBody(json.decodeFromString<CreateSaleRequest>(entry.bodyJson))
            }
        }
        return when (response.status) {
            HttpStatusCode.Created, HttpStatusCode.OK -> {
                val sale = response.body<SaleDto>()
                SyncHttpResult(SyncHttpOutcome.Success, remoteId = sale.id)
            }
            HttpStatusCode.Conflict -> {
                val body = response.bodyAsText()
                if (body.contains("INSUFFICIENT_STOCK", ignoreCase = true)) {
                    SyncHttpResult(SyncHttpOutcome.InsufficientStock, errorCode = "INSUFFICIENT_STOCK")
                } else {
                    SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = "CONFLICT")
                }
            }
            HttpStatusCode.Unauthorized ->
                SyncHttpResult(SyncHttpOutcome.Unauthorized, errorCode = "UNAUTHORIZED")
            else -> SyncHttpResult(SyncHttpOutcome.ClientError, errorCode = response.status.value.toString())
        }
    }

    private suspend inline fun <reified T> getPaginated(path: String): T =
        authorizedGet(path)

    private suspend inline fun <reified T> authorizedGet(path: String): T =
        http.get("$FIELD_API_BASE_URL$path") {
            tokenProvider.accessToken()?.let { header("Authorization", "Bearer $it") }
        }.body()
}
