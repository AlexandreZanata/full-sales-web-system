package com.fullsales.seller.shared.api

import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.request.HttpRequestBuilder
import io.ktor.client.request.get
import io.ktor.client.request.patch
import io.ktor.client.request.post
import io.ktor.client.statement.HttpResponse
import io.ktor.client.statement.bodyAsText
import io.ktor.http.HttpStatusCode
import io.ktor.http.isSuccess
import kotlinx.serialization.json.Json

internal suspend inline fun <reified T> HttpResponse.decodeSuccess(json: Json): T {
    if (!status.isSuccess()) {
        throw apiExceptionFromResponse(json)
    }
    return body()
}

internal suspend fun HttpResponse.apiExceptionFromResponse(json: Json): ApiException {
    val body = runCatching { bodyAsText() }.getOrDefault("")
    val detail = parseApiError(body, json)
        ?: ApiErrorDetail(
            code = "HTTP_${status.value}",
            message = body.ifBlank { status.description },
            correlationId = "00000000-0000-0000-0000-000000000000",
        )
    return ApiException(detail, status.value)
}

internal suspend inline fun <reified T> HttpClient.apiGet(url: String, json: Json): T =
    get(url).decodeSuccess(json)

internal suspend inline fun <reified T> HttpClient.apiPost(
    url: String,
    json: Json,
    crossinline block: HttpRequestBuilder.() -> Unit = {},
): T = post(url, block).decodeSuccess(json)

internal suspend fun HttpClient.apiPostNoContent(
    url: String,
    json: Json,
    block: HttpRequestBuilder.() -> Unit = {},
) {
    val response = post(url, block)
    if (response.status != HttpStatusCode.NoContent && !response.status.isSuccess()) {
        throw response.apiExceptionFromResponse(json)
    }
}

internal suspend inline fun <reified T> HttpClient.apiPatch(
    url: String,
    json: Json,
    crossinline block: HttpRequestBuilder.() -> Unit = {},
): T = patch(url, block).decodeSuccess(json)
