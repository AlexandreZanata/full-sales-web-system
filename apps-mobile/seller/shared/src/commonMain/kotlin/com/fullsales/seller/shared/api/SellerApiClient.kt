package com.fullsales.seller.shared.api

import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CommerceAddress
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LoginRequest
import com.fullsales.seller.shared.model.LoginResponse
import com.fullsales.seller.shared.model.MediaUploadResponse
import com.fullsales.seller.shared.model.MediaUrlResponse
import com.fullsales.seller.shared.model.CursorListProducts
import com.fullsales.seller.shared.model.PaginatedCommerces
import com.fullsales.seller.shared.model.PaginatedSales
import com.fullsales.seller.shared.model.ProductDetail
import com.fullsales.seller.shared.model.RefreshRequest
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.model.StockBalance
import com.fullsales.seller.shared.model.TopSellingProductsResponse
import io.ktor.client.HttpClient
import io.ktor.client.request.forms.MultiPartFormDataContent
import io.ktor.client.request.forms.formData
import io.ktor.client.request.header
import io.ktor.client.request.setBody
import io.ktor.http.ContentType
import io.ktor.http.Headers
import io.ktor.http.HttpHeaders
import io.ktor.http.contentType
import kotlinx.serialization.json.Json

class SellerApiClient(
    private val http: HttpClient,
    private val baseUrl: String = apiBaseUrl,
    private val json: Json = defaultSellerJson(),
) {
    suspend fun login(email: String, password: String): LoginResponse =
        http.apiPost("$baseUrl/auth/login", json) {
            contentType(ContentType.Application.Json)
            setBody(LoginRequest(email, password))
        }

    suspend fun refresh(refreshToken: String): LoginResponse =
        http.apiPost("$baseUrl/auth/refresh", json) {
            contentType(ContentType.Application.Json)
            setBody(RefreshRequest(refreshToken))
        }

    suspend fun logout() {
        http.apiPostNoContent("$baseUrl/auth/logout", json)
    }

    suspend fun getSettings(): SiteSettings =
        http.apiGet("$baseUrl/settings", json)

    suspend fun listCommerces(
        page: Int = 1,
        pageSize: Int = 50,
        active: Boolean? = null,
    ): PaginatedCommerces {
        val params = buildMap {
            active?.let { put("active", it.toString()) }
        }
        val query = paginationQuery(page, pageSize, params)
        return http.apiGet("$baseUrl/commerces?$query", json)
    }

    suspend fun getCommerce(id: String): Commerce =
        http.apiGet("$baseUrl/commerces/$id", json)

    suspend fun listCommerceAddresses(id: String): List<CommerceAddress> =
        http.apiGet("$baseUrl/commerces/$id/addresses", json)

    suspend fun listProducts(
        limit: Int = 50,
        cursor: String? = null,
        active: Boolean = true,
    ): CursorListProducts {
        val params = buildList {
            add("limit=$limit")
            add("filter[active]=$active")
            cursor?.let { add("cursor=$it") }
        }
        return http.apiGet("$baseUrl/products?${params.joinToString("&")}", json)
    }

    suspend fun getProduct(id: String): ProductDetail =
        http.apiGet("$baseUrl/products/$id", json)

    suspend fun listTopSellingProducts(limit: Int = 5): TopSellingProductsResponse =
        http.apiGet("$baseUrl/products/top-selling?limit=$limit", json)

    suspend fun getStockBalance(productId: String): StockBalance =
        http.apiGet("$baseUrl/inventory/products/$productId/balance", json)

    suspend fun createSale(request: CreateSaleRequest, idempotencyKey: String): Sale =
        http.apiPost("$baseUrl/sales", json) {
            contentType(ContentType.Application.Json)
            header("Idempotency-Key", idempotencyKey)
            setBody(request)
        }

    suspend fun listSales(
        page: Int = 1,
        pageSize: Int = 50,
        commerceId: String? = null,
        status: String? = null,
        from: String? = null,
        to: String? = null,
    ): PaginatedSales {
        val params = buildMap {
            commerceId?.let { put("commerceId", it) }
            status?.let { put("status", it) }
            from?.let { put("from", it) }
            to?.let { put("to", it) }
        }
        val query = paginationQuery(page, pageSize, params)
        return http.apiGet("$baseUrl/sales?$query", json)
    }

    suspend fun getSale(id: String): Sale =
        http.apiGet("$baseUrl/sales/$id", json)

    suspend fun confirmSale(id: String): Sale =
        http.apiPost("$baseUrl/sales/$id/confirm", json)

    suspend fun cancelSale(id: String): Sale =
        http.apiPost("$baseUrl/sales/$id/cancel", json)

    suspend fun getMediaUrl(id: String): MediaUrlResponse =
        http.apiGet("$baseUrl/media/$id/url", json)

    suspend fun uploadMedia(
        fileBytes: ByteArray,
        fileName: String,
        mimeType: String,
        entityType: String,
        entityId: String,
    ): MediaUploadResponse = http.apiPost("$baseUrl/media/upload", json) {
        setBody(
            MultiPartFormDataContent(
                formData {
                    append(
                        "file",
                        fileBytes,
                        Headers.build {
                            append(HttpHeaders.ContentType, mimeType)
                            append(HttpHeaders.ContentDisposition, "filename=\"$fileName\"")
                        },
                    )
                    append("entityType", entityType)
                    append("entityId", entityId)
                },
            ),
        )
    }
}
