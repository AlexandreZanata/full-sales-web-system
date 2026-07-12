package com.fullsales.seller.app.ui.sales

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
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
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
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.listItemSummary
import com.fullsales.seller.app.ui.components.SellerEmptyState
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.SalesListItem
import com.fullsales.seller.shared.model.formatMoneyMinorUnits
import com.fullsales.seller.shared.model.formatSalesListDateTime

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
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            val message = if (code == "OFFLINE") s.common.noConnection else code
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
            when {
                state.items.isEmpty() && state.isOffline && !state.remoteLoaded ->
                    SellerEmptyState(
                        title = s.sales.offlineTitle,
                        message = s.sales.offlineMessage,
                        modifier = Modifier.fillMaxSize(),
                    )
                state.items.isEmpty() ->
                    SellerEmptyState(
                        title = s.sales.emptyTitle,
                        message = s.sales.emptyMessage,
                        actionLabel = s.sales.emptyAction,
                        onAction = onNewSale,
                        modifier = Modifier.fillMaxSize(),
                    )
                else -> LazyColumn(
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    items(state.items, key = { it.navigationId }) { sale ->
                        SaleRow(sale = sale, onClick = { onSaleClick(sale.navigationId) })
                    }
                }
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
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .listItemSummary(summary)
            .clickable(onClick = onClick),
        shape = MaterialTheme.shapes.medium,
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(sale.shortId, style = MaterialTheme.typography.labelSmall)
                Text(dateLabel, style = MaterialTheme.typography.bodySmall)
            }
            Column(horizontalAlignment = Alignment.End) {
                SaleStatusChip(status = sale.status)
                Text(money, style = MaterialTheme.typography.titleMedium)
            }
        }
    }
}
