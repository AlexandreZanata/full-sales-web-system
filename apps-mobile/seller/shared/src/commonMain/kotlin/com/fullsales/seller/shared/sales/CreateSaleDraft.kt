package com.fullsales.seller.shared.sales

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class CreateSaleDraft(
    val commerceId: String = "",
    val paymentMethod: String = "",
    val lines: List<CreateSaleLineInput> = listOf(CreateSaleLineInput()),
) {
    fun isEffectivelyEmpty(): Boolean =
        commerceId.isBlank() &&
            paymentMethod.isBlank() &&
            lines.all { line ->
                line.productId.isBlank() &&
                    line.quantityText == "1" &&
                    line.productSearchQuery.isBlank()
            }
}

object CreateSaleDraftCodec {
    private val json = Json { ignoreUnknownKeys = true }

    fun encode(draft: CreateSaleDraft): String = json.encodeToString(draft)

    fun decode(raw: String): CreateSaleDraft? =
        runCatching { json.decodeFromString<CreateSaleDraft>(raw) }.getOrNull()
}

fun createSaleDraftFrom(
    commerceId: String,
    paymentMethod: String,
    lines: List<CreateSaleLineInput>,
): CreateSaleDraft = CreateSaleDraft(
    commerceId = commerceId,
    paymentMethod = paymentMethod,
    lines = lines.map { it.copy(productSearchQuery = "") },
)
