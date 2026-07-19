package com.fullsales.seller.app.ui.commerces

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.listItemSummary
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.app.ui.components.SellerEmptyState
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.ui.ListEmptyDomain
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.listEmptyCopy
import com.fullsales.seller.shared.ui.listSnackbarMessage

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CommerceListScreen(
    viewModel: CommerceViewModel,
    onCommerceClick: (String) -> Unit,
    onPick: ((String) -> Unit)? = null,
    onRegisterCommerce: (() -> Unit)? = null,
    title: String? = null,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            snackbarHostState.showSnackbar(listSnackbarMessage(s, code))
            viewModel.clearSnackbar()
        }
    }
    val content: @Composable () -> Unit = {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(horizontal = 16.dp)
                .padding(top = 4.dp, bottom = 16.dp),
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
            when {
                state.items.isEmpty() &&
                    state.emptyReason != null &&
                    state.emptyReason != ListEmptyReason.RefreshFailedKeepCache -> {
                    val copy = listEmptyCopy(s, state.emptyReason!!, ListEmptyDomain.Commerces)
                    SellerEmptyState(
                        title = copy.title,
                        message = copy.message,
                        actionLabel = onRegisterCommerce?.let { s.commerces.registerFab },
                        onAction = onRegisterCommerce,
                        modifier = Modifier
                            .fillMaxSize()
                            .semantics { contentDescription = copy.announcement },
                    )
                }
                state.isFilterEmpty -> SellerEmptyState(
                    title = s.commerces.emptyTitle,
                    message = s.commerces.empty,
                    modifier = Modifier.fillMaxSize(),
                )
                else -> LazyColumn(contentPadding = androidx.compose.foundation.layout.PaddingValues(bottom = 16.dp)) {
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
    if (onRegisterCommerce != null) {
        NestedScreenScaffold(
            snackbarHost = { SnackbarHost(snackbarHostState) },
            floatingActionButton = {
                FloatingActionButton(onClick = onRegisterCommerce) {
                    Icon(Icons.Default.Add, contentDescription = s.a11y.registerCommerce)
                }
            },
        ) { padding ->
            PullToRefreshBox(
                isRefreshing = state.refreshing,
                onRefresh = { viewModel.refresh() },
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding)
                    .semantics { contentDescription = s.a11y.pullToRefresh },
            ) { content() }
        }
    } else {
        NestedScreenScaffold(
            snackbarHost = { SnackbarHost(snackbarHostState) },
        ) { padding ->
            PullToRefreshBox(
                isRefreshing = state.refreshing,
                onRefresh = { viewModel.refresh() },
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding)
                    .semantics { contentDescription = s.a11y.pullToRefresh },
            ) { content() }
        }
    }
}

@Composable
private fun CommerceRow(commerce: Commerce, onClick: () -> Unit) {
    val s = LocalSellerStrings.current
    val status = if (commerce.active) s.common.active else s.common.inactive
    val summary = SellerStrings.commerceListItem(s, commerce.displayName(), status)
    SellerSurfaceCard(
        modifier = Modifier
            .padding(vertical = 4.dp)
            .listItemSummary(summary),
        onClick = onClick,
        contentPadding = false,
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(14.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(4.dp)) {
                Text(
                    commerce.displayName(),
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
                if (!commerce.tradeName.isNullOrBlank()) {
                    Text(
                        commerce.legalName,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
            if (!commerce.active) {
                Text(
                    s.common.inactive,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}
