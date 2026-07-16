package com.fullsales.seller.shared.api

import com.fullsales.seller.shared.model.CnpjLookupResult
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CommerceAddress
import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.CursorListRegistrations
import com.fullsales.seller.shared.model.PatchRegistrationRequest
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.LoginRequest
import com.fullsales.seller.shared.model.LoginResponse
import com.fullsales.seller.shared.model.MediaUploadResponse
import com.fullsales.seller.shared.model.MediaUrlResponse
import com.fullsales.seller.shared.model.CursorListCommerceAddresses
import com.fullsales.seller.shared.model.CursorListCommerces
import com.fullsales.seller.shared.model.CursorListProducts
import com.fullsales.seller.shared.model.CursorListSales
import com.fullsales.seller.shared.model.ProductDetail
import com.fullsales.seller.shared.model.RefreshRequest
import com.fullsales.seller.shared.model.Sale
import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.model.StockBalance
import com.fullsales.seller.shared.model.TopSellingProductsResponse
import io.ktor.client.HttpClient
import io.ktor.client.request.forms.MultiPartFormDataContent
import io.ktor.client.request.forms.formData
import io.ktor.client.request.get
import io.ktor.client.request.header
import io.ktor.client.request.setBody
import io.ktor.client.statement.HttpResponse
import io.ktor.http.ContentType
import io.ktor.http.Headers
import io.ktor.http.HttpHeaders
import io.ktor.http.contentType
import io.ktor.http.isSuccess
import kotlinx.serialization.json.Json

class SellerApiClient(
    private val http: HttpClient,
    private val baseUrl: String = apiBaseUrl,
    private val json: Json = defaultSellerJson(),
) {
    /** Lightweight reachability probe (`GET /health` beside `/v1`). */
    suspend fun probeReachable(): Boolean = runCatching {
        val origin = baseUrl.removeSuffix("/v1").trimEnd('/')
        val response: HttpResponse = http.get("$origin/health")
        response.status.isSuccess()
    }.getOrDefault(false)

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
        limit: Int = 50,
        cursor: String? = null,
        active: Boolean? = null,
    ): CursorListCommerces {
        val params = buildList {
            add("limit=$limit")
            active?.let { add("filter[active]=$it") }
            cursor?.let { add("cursor=$it") }
        }
        return http.apiGet("$baseUrl/commerces?${params.joinToString("&")}", json)
    }

    suspend fun getCommerce(id: String): Commerce =
        http.apiGet("$baseUrl/commerces/$id", json)

    suspend fun lookupCnpj(cnpj: String): CnpjLookupResult {
        val digits = cnpj.filter { it.isDigit() }
        return http.apiGet("$baseUrl/commerces/cnpj-lookup?cnpj=$digits", json)
    }

    suspend fun submitRegistration(
        request: SubmitRegistrationRequest,
        idempotencyKey: String? = null,
    ): CommerceRegistration =
        http.apiPost("$baseUrl/commerces/registrations", json) {
            contentType(ContentType.Application.Json)
            idempotencyKey?.let { header("Idempotency-Key", it) }
            setBody(request)
        }

    suspend fun listRegistrations(
        limit: Int = 50,
        cursor: String? = null,
        status: String? = null,
    ): CursorListRegistrations {
        val params = buildList {
            add("limit=$limit")
            status?.let { add("filter[status]=$it") }
            cursor?.let { add("cursor=$it") }
        }
        return http.apiGet("$baseUrl/commerces/registrations?${params.joinToString("&")}", json)
    }

    suspend fun getRegistration(id: String): CommerceRegistration =
        http.apiGet("$baseUrl/commerces/registrations/$id", json)

    suspend fun patchRegistration(id: String, request: PatchRegistrationRequest): CommerceRegistration =
        http.apiPatch("$baseUrl/commerces/registrations/$id", json) {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

    suspend fun listCommerceAddresses(id: String): List<CommerceAddress> {
        val all = mutableListOf<CommerceAddress>()
        var cursor: String? = null
        while (true) {
            val params = buildList {
                add("limit=100")
                cursor?.let { add("cursor=$it") }
            }
            val page = http.apiGet<CursorListCommerceAddresses>(
                "$baseUrl/commerces/$id/addresses?${params.joinToString("&")}",
                json,
            )
            all += page.data
            if (!page.pagination.hasMore || page.pagination.nextCursor == null) break
            cursor = page.pagination.nextCursor
            if (page.data.size < 100) break
        }
        return all
    }

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
        limit: Int = 50,
        cursor: String? = null,
        commerceId: String? = null,
        status: String? = null,
        from: String? = null,
        to: String? = null,
    ): CursorListSales {
        val params = buildList {
            add("limit=$limit")
            commerceId?.let { add("filter[commerce_id]=$it") }
            status?.let { add("filter[status]=$it") }
            from?.let { add("filter[created_at][gte]=$it") }
            to?.let { add("filter[created_at][lte]=$it") }
            cursor?.let { add("cursor=$it") }
        }
        return http.apiGet("$baseUrl/sales?${params.joinToString("&")}", json)
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
