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
    val state by viewModel.state.collectAsState()
    Scaffold(
        floatingActionButton = {
            FloatingActionButton(onClick = onNewSale) {
                Icon(Icons.Default.Add, contentDescription = "New sale")
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
                        title = "Offline",
                        message = "Connect to the network and pull to refresh to load sales.",
                        modifier = Modifier.fillMaxSize(),
                    )
                state.items.isEmpty() ->
                    SellerEmptyState(
                        title = "No sales yet",
                        message = "Create your first sale to get started.",
                        actionLabel = "Create sale",
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
    val money = NumberFormat.getCurrencyInstance(Locale("pt", "BR")).format(sale.totalAmount)
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
