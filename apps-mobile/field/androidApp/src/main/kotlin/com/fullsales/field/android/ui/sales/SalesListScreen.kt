package com.fullsales.field.android.ui.sales

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.field.shared.model.Sale
import java.text.NumberFormat
import java.util.Locale

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SalesListScreen(
    viewModel: SalesViewModel,
    onNewSale: () -> Unit,
) {
    val sales by viewModel.sales.collectAsState()
    Scaffold(
        topBar = {
            TopAppBar(title = { Text("Sales") })
        },
        floatingActionButton = {},
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Button(onClick = onNewSale, modifier = Modifier.fillMaxWidth()) {
                Text("New sale")
            }
            if (sales.isEmpty()) {
                Text("No sales yet", style = MaterialTheme.typography.bodyLarge)
            } else {
                LazyColumn(contentPadding = PaddingValues(bottom = 16.dp)) {
                    items(sales, key = { it.localId }) { sale ->
                        SaleRow(sale)
                    }
                }
            }
        }
    }
}

@Composable
private fun SaleRow(sale: Sale) {
    val money = NumberFormat.getCurrencyInstance(Locale("pt", "BR"))
        .format(sale.totalAmount)
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp)
            .clickable { },
    ) {
        Column(modifier = Modifier.padding(12.dp)) {
            Text(sale.localId.take(8), style = MaterialTheme.typography.labelSmall)
            Text(sale.status.name, style = MaterialTheme.typography.titleMedium)
            Text(money, style = MaterialTheme.typography.bodyLarge)
        }
    }
}
