package com.fullsales.seller.shared.model

data class CommerceAddressUi(
    val id: String,
    val typeLabel: String,
    val streetLine: String,
    val cityLine: String,
    val isPrimary: Boolean,
)

fun CommerceAddress.toUiModel(): CommerceAddressUi = CommerceAddressUi(
    id = id,
    typeLabel = addressTypeLabel(type),
    streetLine = formatStreetLine(street, number),
    cityLine = formatCityLine(city, state, postalCode),
    isPrimary = isPrimary,
)

fun Commerce.displayName(): String = tradeName?.takeIf { it.isNotBlank() } ?: legalName

fun maskCnpj(raw: String): String {
    val clean = raw.filter { it.isLetterOrDigit() }
    if (clean.length < 4) return "****"
    return "${clean.take(2)}.***.***/****-${clean.takeLast(2)}"
}

private fun addressTypeLabel(type: String): String = when (type) {
    "Billing" -> "Billing"
    "Delivery" -> "Delivery"
    else -> type
}

private fun formatStreetLine(street: String, number: String): String =
    listOf(street.trim(), number.trim()).filter { it.isNotEmpty() }.joinToString(", ")

private fun formatCityLine(city: String, state: String, postalCode: String): String {
    val cityState = listOf(city.trim(), state.trim()).filter { it.isNotEmpty() }.joinToString(" — ")
    return listOf(cityState, postalCode.trim()).filter { it.isNotEmpty() }.joinToString(" ")
}
