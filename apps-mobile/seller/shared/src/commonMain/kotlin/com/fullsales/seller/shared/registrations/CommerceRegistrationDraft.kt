package com.fullsales.seller.shared.registrations

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class CommerceRegistrationDraft(
    val mode: String = "manual",
    val cnpj: String = "",
    val legalName: String = "",
    val tradeName: String = "",
    val phone: String = "",
    val email: String = "",
    val street: String = "",
    val number: String = "",
    val district: String = "",
    val city: String = "",
    val state: String = "",
    val postalCode: String = "",
    val lookupSnapshotJson: String? = null,
) {
    fun isEffectivelyEmpty(): Boolean =
        cnpj.isBlank() &&
            legalName.isBlank() &&
            tradeName.isBlank() &&
            phone.isBlank() &&
            email.isBlank() &&
            street.isBlank() &&
            number.isBlank() &&
            district.isBlank() &&
            city.isBlank() &&
            state.isBlank() &&
            postalCode.isBlank()
}

object CommerceRegistrationDraftCodec {
    private val json = Json { ignoreUnknownKeys = true }

    fun encode(draft: CommerceRegistrationDraft): String = json.encodeToString(draft)

    fun decode(raw: String): CommerceRegistrationDraft? =
        runCatching { json.decodeFromString<CommerceRegistrationDraft>(raw) }.getOrNull()
}
