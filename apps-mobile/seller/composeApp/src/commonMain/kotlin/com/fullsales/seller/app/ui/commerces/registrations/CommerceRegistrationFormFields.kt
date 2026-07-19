package com.fullsales.seller.app.ui.commerces.registrations

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.SellerSectionTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.registrations.CommerceRegistrationDraft
import com.fullsales.seller.shared.registrations.CommerceRegistrationFormErrors

@Composable
fun CommerceRegistrationFormFields(
    draft: CommerceRegistrationDraft,
    errors: CommerceRegistrationFormErrors,
    cnpjReadOnly: Boolean,
    onDraftChange: ((CommerceRegistrationDraft) -> CommerceRegistrationDraft) -> Unit,
) {
    val s = LocalSellerStrings.current
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        OutlinedTextField(
            value = draft.cnpj,
            onValueChange = { value -> onDraftChange { it.copy(cnpj = value) } },
            label = { Text(s.registrations.cnpjLabel) },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
            readOnly = cnpjReadOnly,
            isError = errors.cnpjError != null,
            supportingText = errors.cnpjError?.let {
                { Text(SellerStrings.registrationFieldError(s, it)) }
            },
        )
        OutlinedTextField(
            value = draft.legalName,
            onValueChange = { value -> onDraftChange { it.copy(legalName = value) } },
            label = { Text(s.registrations.legalName) },
            modifier = Modifier.fillMaxWidth(),
            isError = errors.legalNameError != null,
            supportingText = errors.legalNameError?.let {
                { Text(SellerStrings.registrationFieldError(s, it)) }
            },
        )
        OutlinedTextField(
            value = draft.tradeName,
            onValueChange = { value -> onDraftChange { it.copy(tradeName = value) } },
            label = { Text(s.registrations.tradeName) },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
        )
        OutlinedTextField(
            value = draft.phone,
            onValueChange = { value -> onDraftChange { it.copy(phone = value) } },
            label = { Text(s.registrations.phone) },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
        )
        OutlinedTextField(
            value = draft.email,
            onValueChange = { value -> onDraftChange { it.copy(email = value) } },
            label = { Text(s.registrations.email) },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
        )
        SellerSectionTitle(s.registrations.addressSection)
        AddressField(draft.street, errors.streetError, s.registrations.street) { value ->
            onDraftChange { it.copy(street = value) }
        }
        AddressField(draft.number, errors.numberError, s.registrations.number) { value ->
            onDraftChange { it.copy(number = value) }
        }
        OutlinedTextField(
            value = draft.district,
            onValueChange = { value -> onDraftChange { it.copy(district = value) } },
            label = { Text(s.registrations.district) },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
        )
        AddressField(draft.city, errors.cityError, s.registrations.city) { value ->
            onDraftChange { it.copy(city = value) }
        }
        AddressField(draft.state, errors.stateError, s.registrations.state) { value ->
            onDraftChange { it.copy(state = value) }
        }
        AddressField(draft.postalCode, errors.postalCodeError, s.registrations.postalCode) { value ->
            onDraftChange { it.copy(postalCode = value) }
        }
    }
}

@Composable
private fun AddressField(
    value: String,
    errorCode: String?,
    label: String,
    onChange: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    OutlinedTextField(
        value = value,
        onValueChange = onChange,
        label = { Text(label) },
        modifier = Modifier.fillMaxWidth(),
        singleLine = true,
        isError = errorCode != null,
        supportingText = errorCode?.let {
            { Text(SellerStrings.registrationFieldError(s, it)) }
        },
    )
}
