package com.fullsales.seller.app.ui.commerces.registrations

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.listItemSummary
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.shared.model.maskCnpj

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MyRegistrationsScreen(viewModel: MyRegistrationsViewModel) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    PullToRefreshBox(
        isRefreshing = state.refreshing,
        onRefresh = { viewModel.refresh() },
        modifier = Modifier
            .fillMaxSize()
            .semantics { contentDescription = s.a11y.pullToRefresh },
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(s.registrations.myTitle, style = MaterialTheme.typography.headlineSmall, modifier = Modifier.screenTitle())
            state.error?.let { Text(it, color = MaterialTheme.colorScheme.error) }
            if (state.isOffline) Text(s.common.offline, color = MaterialTheme.colorScheme.error)
            when {
                state.isEmpty -> Text(s.registrations.empty, style = MaterialTheme.typography.bodyLarge)
                else -> LazyColumn(contentPadding = PaddingValues(bottom = 16.dp)) {
                    items(state.items, key = { it.id }) { item ->
                        RegistrationRow(item)
                    }
                }
            }
        }
    }
}

@Composable
private fun RegistrationRow(item: CommerceRegistration) {
    val s = LocalSellerStrings.current
    val status = SellerStrings.registrationStatus(s, item.registrationStatus)
    val summary = SellerStrings.registrationListItem(
        s,
        item.displayName(),
        status,
        maskCnpj(item.cnpj),
    )
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp)
            .listItemSummary(summary),
    ) {
        Column(modifier = Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
            Text(item.displayName(), style = MaterialTheme.typography.titleMedium)
            Text(maskCnpj(item.cnpj), style = MaterialTheme.typography.bodySmall)
            Text(status, style = MaterialTheme.typography.labelMedium)
            item.rejectionReason?.takeIf { it.isNotBlank() }?.let { reason ->
                Text(
                    SellerStrings.format(s.registrations.rejectionReason, "reason" to reason),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.error,
                )
            }
        }
    }
}
