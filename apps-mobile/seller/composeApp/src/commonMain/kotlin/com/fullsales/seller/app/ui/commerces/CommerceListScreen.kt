package com.fullsales.seller.app.ui.commerces

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.listItemSummary
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.displayName

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CommerceListScreen(
    viewModel: CommerceViewModel,
    onCommerceClick: (String) -> Unit,
    onPick: ((String) -> Unit)? = null,
    title: String? = null,
) {
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
            Text(
                title ?: s.commerces.title,
                style = MaterialTheme.typography.headlineSmall,
                modifier = Modifier.screenTitle(),
            )
            OutlinedTextField(
                value = state.searchQuery,
                onValueChange = viewModel::setSearchQuery,
                label = { Text(s.commerces.searchByName) },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true,
            )
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                FilterChip(
                    selected = state.activeOnly,
                    onClick = { viewModel.setActiveOnly(true) },
                    label = { Text(s.common.active) },
                    modifier = Modifier.selectableChipA11y(s.common.active, state.activeOnly, s.a11y.selected),
                )
                FilterChip(
                    selected = !state.activeOnly,
                    onClick = { viewModel.setActiveOnly(false) },
                    label = { Text(s.common.all) },
                    modifier = Modifier.selectableChipA11y(s.common.all, !state.activeOnly, s.a11y.selected),
                )
            }
            state.error?.let { Text(it, color = MaterialTheme.colorScheme.error) }
            when {
                state.isEmpty -> Text(
                    if (state.isOffline && state.items.isEmpty()) s.commerces.emptyOffline else s.commerces.empty,
                    style = MaterialTheme.typography.bodyLarge,
                )
                else -> LazyColumn(contentPadding = PaddingValues(bottom = 16.dp)) {
                    items(state.filtered, key = { it.id }) { commerce ->
                        CommerceRow(
                            commerce = commerce,
                            onClick = {
                                if (onPick != null) onPick(commerce.id) else onCommerceClick(commerce.id)
                            },
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun CommerceRow(commerce: Commerce, onClick: () -> Unit) {
    val s = LocalSellerStrings.current
    val status = if (commerce.active) s.common.active else s.common.inactive
    val summary = SellerStrings.commerceListItem(s, commerce.displayName(), status)
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp)
            .listItemSummary(summary)
            .clickable(onClick = onClick),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(commerce.displayName(), style = MaterialTheme.typography.titleMedium)
                if (!commerce.tradeName.isNullOrBlank()) {
                    Text(commerce.legalName, style = MaterialTheme.typography.bodySmall)
                }
            }
            if (!commerce.active) {
                Text(s.common.inactive, style = MaterialTheme.typography.labelSmall)
            }
        }
    }
}
