package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
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
import androidx.compose.material3.MaterialTheme
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
import com.fullsales.seller.app.ui.components.SellerEmptyState
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.SalesListItem
import com.fullsales.seller.shared.model.formatMoneyMinorUnits
import com.fullsales.seller.shared.model.formatSalesListDateTime
import com.fullsales.seller.shared.ui.ListEmptyDomain
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.listEmptyCopy
import com.fullsales.seller.shared.ui.listSnackbarMessage

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SalesListScreen(
    viewModel: SalesListViewModel,
    onSaleClick: (String) -> Unit,
    onNewSale: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(Unit) {
        viewModel.reloadCatalogShare()
    }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            val message = when (code) {
                "CATALOG_COPIED" -> s.sales.catalogLinkCopied
                else -> listSnackbarMessage(s, code)
            }
            snackbarHostState.showSnackbar(message)
            viewModel.clearSnackbar()
        }
    }
    NestedScreenScaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) },
        floatingActionButton = {
            FloatingActionButton(onClick = onNewSale) {
                Icon(Icons.Default.Add, contentDescription = s.a11y.newSale)
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
        ) {
            Column(modifier = Modifier.fillMaxSize()) {
                CatalogShareBar(
                    catalogUrl = state.catalogUrl,
                    enabled = state.catalogShareActive,
                    onShare = viewModel::shareCatalogLink,
                    onCopy = viewModel::copyCatalogLink,
                    onOpen = viewModel::openCatalogLink,
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
                )
                SalesListBody(
                    state = state,
                    onSaleClick = onSaleClick,
                    onNewSale = onNewSale,
                    modifier = Modifier.weight(1f),
                )
            }
        }
    }
}

@Composable
private fun SalesListBody(
    state: SalesListUiState,
    onSaleClick: (String) -> Unit,
    onNewSale: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    val reason = state.emptyReason
    when {
        state.items.isEmpty() && reason != null && reason != ListEmptyReason.RefreshFailedKeepCache -> {
            val copy = listEmptyCopy(s, reason, ListEmptyDomain.Sales)
            SellerEmptyState(
                title = copy.title,
                message = copy.message,
                actionLabel = if (reason == ListEmptyReason.SyncedEmpty) s.sales.emptyAction else null,
                onAction = if (reason == ListEmptyReason.SyncedEmpty) onNewSale else null,
                modifier = modifier
                    .fillMaxSize()
                    .semantics { contentDescription = copy.announcement },
            )
        }
        else -> LazyColumn(
            modifier = modifier.fillMaxSize(),
            contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            items(state.items, key = { it.navigationId }) { sale ->
                SaleRow(sale = sale, onClick = { onSaleClick(sale.navigationId) })
            }
        }
    }
}

@Composable
private fun SaleRow(sale: SalesListItem, onClick: () -> Unit) {
    val s = LocalSellerStrings.current
    val money = formatMoneyMinorUnits(sale.totalAmount.toLong(), sale.totalCurrency)
    val dateLabel = formatSalesListDateTime(sale.createdAtEpochMs)
    val statusLabel = SellerStrings.saleStatus(s, sale.status)
    val summary = SellerStrings.saleListItem(s, sale.shortId, dateLabel, statusLabel, money)
    SellerSurfaceCard(
        modifier = Modifier.listItemSummary(summary),
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
                    sale.shortId,
                    style = MaterialTheme.typography.titleSmall,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    dateLabel,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Column(horizontalAlignment = Alignment.End, verticalArrangement = Arrangement.spacedBy(4.dp)) {
                SaleStatusChip(status = sale.status)
                Text(
                    money,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.primary,
                )
            }
        }
    }
}
