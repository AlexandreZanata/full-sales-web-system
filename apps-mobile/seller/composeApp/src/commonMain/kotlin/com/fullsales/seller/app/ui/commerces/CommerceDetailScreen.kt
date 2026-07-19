package com.fullsales.seller.app.ui.commerces

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.SellerEmptyState
import com.fullsales.seller.app.ui.components.SellerSectionTitle
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.CommerceAddressUi
import com.fullsales.seller.shared.model.maskCnpj

@Composable
fun CommerceDetailScreen(
    commerceId: String,
    viewModel: CommerceDetailViewModel,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    LaunchedEffect(commerceId) { viewModel.load(commerceId) }

    when {
        state.loading -> CircularProgressIndicator(modifier = Modifier.padding(24.dp))
        state.errorCode != null -> SellerEmptyState(
            title = SellerStrings.commerceError(s, state.errorCode!!),
            message = s.common.tryAgain,
            modifier = Modifier.fillMaxSize(),
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
    val s = LocalSellerStrings.current
    LazyColumn(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        item {
            SellerSurfaceCard(highlighted = true, contentPadding = false) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    Text(
                        legalName,
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold,
                        color = MaterialTheme.colorScheme.onPrimaryContainer,
                    )
                    tradeName?.takeIf { it.isNotBlank() }?.let {
                        Text(
                            it,
                            style = MaterialTheme.typography.titleMedium,
                            color = MaterialTheme.colorScheme.onPrimaryContainer,
                        )
                    }
                    cnpj?.let {
                        Text(
                            SellerStrings.format(s.commerces.cnpjLabel, "value" to maskCnpj(it)),
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onPrimaryContainer,
                        )
                    }
                    Text(
                        if (active) s.common.active else s.common.inactive,
                        style = MaterialTheme.typography.labelLarge,
                        fontWeight = FontWeight.SemiBold,
                        color = MaterialTheme.colorScheme.onPrimaryContainer,
                    )
                }
            }
        }
        item { SellerSectionTitle(s.commerces.addresses) }
        if (addresses.isEmpty()) {
            item {
                SellerEmptyState(
                    title = s.commerces.noAddresses,
                    message = s.commerces.empty,
                    modifier = Modifier.fillMaxWidth(),
                )
            }
        } else {
            items(addresses, key = { it.id }) { address ->
                AddressRow(address)
            }
        }
    }
}

@Composable
private fun AddressRow(address: CommerceAddressUi) {
    val s = LocalSellerStrings.current
    SellerSurfaceCard(contentPadding = false) {
        Column(
            modifier = Modifier.padding(14.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(
                    address.typeLabel,
                    style = MaterialTheme.typography.titleSmall,
                    fontWeight = FontWeight.SemiBold,
                )
                if (address.isPrimary) {
                    Text(
                        s.common.primary,
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.primary,
                    )
                }
            }
            Text(address.streetLine, style = MaterialTheme.typography.bodyMedium)
            Text(
                address.cityLine,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}
