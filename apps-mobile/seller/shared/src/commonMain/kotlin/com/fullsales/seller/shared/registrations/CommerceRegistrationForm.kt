package com.fullsales.seller.shared.registrations

import com.fullsales.seller.shared.model.CnpjLookupResult
import com.fullsales.seller.shared.model.DeliveryAddressRequest
import com.fullsales.seller.shared.model.RegistrationContact
import com.fullsales.seller.shared.model.RegistrationMode
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.cnpjDigitsOnly
import com.fullsales.seller.shared.model.isValidCnpjInput
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement

data class CommerceRegistrationFormErrors(
    val cnpjError: String? = null,
    val legalNameError: String? = null,
    val streetError: String? = null,
    val numberError: String? = null,
    val cityError: String? = null,
    val stateError: String? = null,
    val postalCodeError: String? = null,
) {
    val isValid: Boolean get() = listOf(
        cnpjError, legalNameError, streetError, numberError, cityError, stateError, postalCodeError,
    ).all { it == null }
}

fun draftFromLookup(result: CnpjLookupResult, snapshotJson: String): CommerceRegistrationDraft =
    CommerceRegistrationDraft(
        mode = RegistrationMode.CNPJ_LOOKUP,
        cnpj = result.cnpj,
        legalName = result.legalName,
        tradeName = result.tradeName,
        street = result.address.street,
        number = result.address.number,
        district = result.address.district,
        city = result.address.city,
        state = result.address.state,
        postalCode = result.address.postalCode,
        lookupSnapshotJson = snapshotJson,
    )

fun validateCommerceRegistrationForm(draft: CommerceRegistrationDraft): CommerceRegistrationFormErrors {
    val cnpjError = when {
        draft.cnpj.isBlank() -> "required"
        !isValidCnpjInput(draft.cnpj) -> "invalid"
        else -> null
    }
    val legalNameError = if (draft.legalName.isBlank()) "required" else null
    val streetError = if (draft.street.isBlank()) "required" else null
    val numberError = if (draft.number.isBlank()) "required" else null
    val cityError = if (draft.city.isBlank()) "required" else null
    val stateError = if (draft.state.isBlank()) "required" else null
    val postalCodeError = if (draft.postalCode.isBlank()) "required" else null
    return CommerceRegistrationFormErrors(
        cnpjError = cnpjError,
        legalNameError = legalNameError,
        streetError = streetError,
        numberError = numberError,
        cityError = cityError,
        stateError = stateError,
        postalCodeError = postalCodeError,
    )
}

fun buildSubmitRegistrationRequest(draft: CommerceRegistrationDraft): SubmitRegistrationRequest {
    val snapshot: JsonElement? = draft.lookupSnapshotJson?.let { raw ->
        Json.parseToJsonElement(raw)
    }
    return SubmitRegistrationRequest(
        cnpj = cnpjDigitsOnly(draft.cnpj),
        legalName = draft.legalName.trim(),
        tradeName = draft.tradeName.trim().takeIf { it.isNotEmpty() },
        contact = RegistrationContact(
            phone = draft.phone.trim().takeIf { it.isNotEmpty() },
            email = draft.email.trim().takeIf { it.isNotEmpty() },
        ),
        deliveryAddress = DeliveryAddressRequest(
            street = draft.street.trim(),
            number = draft.number.trim(),
            district = draft.district.trim().takeIf { it.isNotEmpty() },
            city = draft.city.trim(),
            state = draft.state.trim(),
            postalCode = draft.postalCode.trim(),
            isPrimary = true,
        ),
        registrationMode = draft.mode,
        lookupSnapshot = snapshot,
    )
}
