package com.fullsales.seller.android.ui.sales

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
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.android.ui.components.SellerEmptyState
import com.fullsales.seller.android.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.model.SalesListItem
import java.text.NumberFormat
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SalesListScreen(
    viewModel: SalesListViewModel,
    onSaleClick: (String) -> Unit,
    onNewSale: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    Scaffold(
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
                .padding(padding),
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
    val moneyLocale = if (sale.totalCurrency == "BRL") Locale("pt", "BR") else Locale.getDefault()
    val money = NumberFormat.getCurrencyInstance(moneyLocale).format(sale.totalAmount)
    val dateLabel = SimpleDateFormat("dd/MM/yyyy HH:mm", Locale.getDefault())
        .format(Date(sale.createdAtEpochMs))
    Card(
        modifier = Modifier
            .fillMaxWidth()
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
