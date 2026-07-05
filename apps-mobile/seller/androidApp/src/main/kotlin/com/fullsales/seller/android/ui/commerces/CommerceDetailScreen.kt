package com.fullsales.seller.android.ui.commerces

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.shared.model.CommerceAddressUi
import com.fullsales.seller.shared.model.maskCnpj

@Composable
fun CommerceDetailScreen(
    commerceId: String,
    viewModel: CommerceDetailViewModel,
) {
    val state by viewModel.state.collectAsState()
    LaunchedEffect(commerceId) { viewModel.load(commerceId) }

    when {
        state.loading -> CircularProgressIndicator(modifier = Modifier.padding(24.dp))
        state.error != null -> Text(
            state.error!!,
            color = MaterialTheme.colorScheme.error,
            modifier = Modifier.padding(16.dp),
        )
        state.commerce != null -> CommerceDetailContent(
            legalName = state.commerce!!.legalName,
            tradeName = state.commerce!!.tradeName,
            cnpj = state.commerce!!.cnpj,
            active = state.commerce!!.active,
            addresses = state.addresses,
        )
    }
}

@Composable
private fun CommerceDetailContent(
    legalName: String,
    tradeName: String?,
    cnpj: String?,
    active: Boolean,
    addresses: List<CommerceAddressUi>,
) {
    LazyColumn(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        item {
            Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
                Text(legalName, style = MaterialTheme.typography.headlineSmall)
                tradeName?.takeIf { it.isNotBlank() }?.let {
                    Text(it, style = MaterialTheme.typography.titleMedium)
                }
                cnpj?.let { Text("CNPJ: ${maskCnpj(it)}", style = MaterialTheme.typography.bodyMedium) }
                Text(
                    if (active) "Active" else "Inactive",
                    style = MaterialTheme.typography.labelLarge,
                    color = if (active) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.outline,
                )
            }
        }
        item { Text("Addresses", style = MaterialTheme.typography.titleMedium) }
        if (addresses.isEmpty()) {
            item { Text("No addresses", style = MaterialTheme.typography.bodyMedium) }
        } else {
            items(addresses, key = { it.id }) { address ->
                AddressRow(address)
            }
        }
    }
}

@Composable
private fun AddressRow(address: CommerceAddressUi) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(address.typeLabel, style = MaterialTheme.typography.titleSmall)
                if (address.isPrimary) {
                    Text("Primary", style = MaterialTheme.typography.labelSmall)
                }
            }
            Text(address.streetLine, style = MaterialTheme.typography.bodyMedium)
            Text(address.cityLine, style = MaterialTheme.typography.bodySmall)
        }
    }
}
