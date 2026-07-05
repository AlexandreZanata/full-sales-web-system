package com.fullsales.seller.shared.api

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class ApiErrorBody(
    val error: ApiErrorDetail,
)

@Serializable
data class ApiErrorDetail(
    val code: String,
    val message: String,
    val correlationId: String,
)

class ApiException(
    val detail: ApiErrorDetail,
    val statusCode: Int,
) : Exception(detail.message)

fun parseApiError(body: String, json: Json): ApiErrorDetail? = runCatching {
    json.decodeFromString<ApiErrorBody>(body).error
}.getOrNull()

fun paginationQuery(
    page: Int,
    pageSize: Int,
    params: Map<String, String> = emptyMap(),
): String {
    val parts = buildList {
        add("page=$page")
        add("pageSize=$pageSize")
        params.forEach { (key, value) -> add("$key=$value") }
    }
    return parts.joinToString("&")
}
