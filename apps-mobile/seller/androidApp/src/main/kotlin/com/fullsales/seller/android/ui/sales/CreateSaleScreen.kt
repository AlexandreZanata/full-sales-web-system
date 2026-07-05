package com.fullsales.seller.android.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun CreateSaleScreen(
    selectedCommerceLabel: String?,
    onOpenCommercePicker: () -> Unit,
    onBrowseCommerces: () -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text("New sale", style = MaterialTheme.typography.headlineSmall)
        Text(
            "Select a commerce before adding products (Phase 60).",
            style = MaterialTheme.typography.bodyMedium,
        )
        Button(onClick = onOpenCommercePicker, modifier = Modifier.fillMaxWidth()) {
            Text(
                selectedCommerceLabel?.let { "Commerce: $it" } ?: "Select commerce",
            )
        }
        OutlinedButton(onClick = onBrowseCommerces, modifier = Modifier.fillMaxWidth()) {
            Text("Browse commerces")
        }
    }
}
