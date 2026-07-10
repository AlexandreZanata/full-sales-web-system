package com.fullsales.seller.shared.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement

@Serializable
data class RegistrationContact(
    val phone: String? = null,
    val email: String? = null,
)

@Serializable
data class DeliveryAddressRequest(
    val street: String,
    val number: String,
    val district: String? = null,
    val city: String,
    val state: String,
    @SerialName("postalCode") val postalCode: String,
    @SerialName("isPrimary") val isPrimary: Boolean = true,
)

@Serializable
data class SubmitRegistrationRequest(
    val cnpj: String,
    @SerialName("legalName") val legalName: String,
    @SerialName("tradeName") val tradeName: String? = null,
    val contact: RegistrationContact,
    @SerialName("deliveryAddress") val deliveryAddress: DeliveryAddressRequest,
    @SerialName("registrationMode") val registrationMode: String,
    @SerialName("lookupSnapshot") val lookupSnapshot: JsonElement? = null,
)

@Serializable
data class PatchRegistrationRequest(
    @SerialName("legalName") val legalName: String? = null,
    @SerialName("tradeName") val tradeName: String? = null,
    val contact: RegistrationContact? = null,
    @SerialName("deliveryAddress") val deliveryAddress: DeliveryAddressRequest? = null,
)

@Serializable
data class CommerceRegistration(
    val id: String,
    val cnpj: String,
    @SerialName("legalName") val legalName: String,
    @SerialName("tradeName") val tradeName: String,
    val active: Boolean,
    @SerialName("registrationStatus") val registrationStatus: String,
    @SerialName("rejectionReason") val rejectionReason: String? = null,
    @SerialName("registrationMode") val registrationMode: String? = null,
)

@Serializable
data class CnpjLookupCnae(
    val code: String,
    val description: String,
)

@Serializable
data class CnpjLookupPartner(
    val name: String,
    val role: String? = null,
    @SerialName("joinedAt") val joinedAt: String? = null,
)

@Serializable
data class CnpjLookupAddress(
    val street: String,
    val number: String,
    val complement: String? = null,
    val district: String,
    val city: String,
    val state: String,
    @SerialName("postalCode") val postalCode: String,
)

@Serializable
data class CnpjLookupResult(
    val cnpj: String,
    @SerialName("legalName") val legalName: String,
    @SerialName("tradeName") val tradeName: String,
    val address: CnpjLookupAddress,
    val phone: String? = null,
    val email: String? = null,
    @SerialName("registrationStatus") val registrationStatus: String? = null,
    @SerialName("mainCnae") val mainCnae: CnpjLookupCnae? = null,
    val partners: List<CnpjLookupPartner>? = null,
    @SerialName("upstreamSnapshot") val upstreamSnapshot: JsonElement? = null,
    val provider: String,
    @SerialName("fetchedAt") val fetchedAt: String,
)

object RegistrationMode {
    const val CNPJ_LOOKUP = "cnpj_lookup"
    const val MANUAL = "manual"
}

fun CommerceRegistration.displayName(): String =
    tradeName.takeIf { it.isNotBlank() } ?: legalName

fun cnpjDigitsOnly(raw: String): String = raw.filter { it.isDigit() }

fun isValidCnpjInput(raw: String): Boolean = cnpjDigitsOnly(raw).length == 14
