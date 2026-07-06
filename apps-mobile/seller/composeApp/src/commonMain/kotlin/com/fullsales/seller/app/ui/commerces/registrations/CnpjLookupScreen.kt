package com.fullsales.seller.app.ui.commerces.registrations

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.CnpjLookupResult

@Composable
fun CnpjLookupScreen(
    viewModel: CnpjLookupViewModel,
    onContinue: (CnpjLookupResult) -> Unit,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text(s.registrations.cnpjLookupTitle, style = MaterialTheme.typography.headlineSmall, modifier = Modifier.screenTitle())
        OutlinedTextField(
            value = state.cnpj,
            onValueChange = viewModel::setCnpj,
            label = { Text(s.registrations.cnpjLabel) },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
            isError = state.errorCode != null,
            supportingText = state.errorCode?.let {
                { Text(SellerStrings.registrationError(s, it)) }
            },
        )
        Button(
            onClick = { viewModel.lookup(onContinue) },
            enabled = !state.loading,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text(if (state.loading) s.common.loading else s.registrations.lookupAction)
        }
        if (state.loading) CircularProgressIndicator()
    }
}
