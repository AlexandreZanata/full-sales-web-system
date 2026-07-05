package com.fullsales.seller.android.ui.sales

import androidx.compose.foundation.clickable
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
import androidx.compose.ui.unit.dp
import com.fullsales.seller.android.ui.sync.SyncStatusViewModel
import com.fullsales.seller.shared.model.LocalSale
import java.text.NumberFormat
import java.util.Locale

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SalesListScreen(
    salesViewModel: SalesViewModel,
    syncViewModel: SyncStatusViewModel,
    onSaleClick: (String) -> Unit,
) {
    val sales by salesViewModel.sales.collectAsState()
    val refreshing by syncViewModel.refreshing.collectAsState()
    PullToRefreshBox(
        isRefreshing = refreshing,
        onRefresh = { syncViewModel.refreshNow() },
        modifier = Modifier.fillMaxSize(),
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            if (sales.isEmpty()) {
                Text("No sales yet", style = MaterialTheme.typography.bodyLarge)
            } else {
                LazyColumn(contentPadding = PaddingValues(bottom = 16.dp)) {
                    items(sales, key = { it.localId }) { sale ->
                        SaleRow(sale = sale, onClick = { onSaleClick(sale.localId) })
                    }
                }
            }
        }
    }
}

@Composable
private fun SaleRow(sale: LocalSale, onClick: () -> Unit) {
    val money = NumberFormat.getCurrencyInstance(Locale("pt", "BR")).format(sale.totalAmount)
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp)
            .clickable(onClick = onClick),
    ) {
        Column(modifier = Modifier.padding(12.dp)) {
            Text(sale.localId.take(8), style = MaterialTheme.typography.labelSmall)
            Text(sale.status.name, style = MaterialTheme.typography.titleMedium)
            Text(money, style = MaterialTheme.typography.bodyLarge)
        }
    }
}
