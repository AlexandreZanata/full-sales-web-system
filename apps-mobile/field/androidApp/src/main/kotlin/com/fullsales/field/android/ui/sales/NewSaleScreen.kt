package com.fullsales.field.android.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

private val PAYMENT_METHODS = listOf("cash", "pix", "credit", "debit")

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun NewSaleScreen(
    viewModel: SalesViewModel,
    onBack: () -> Unit,
    onCreated: () -> Unit,
) {
    val state by viewModel.newSale.collectAsState()
    var commerceId by remember { mutableStateOf("") }
    var paymentMethod by remember { mutableStateOf("") }
    var productId by remember { mutableStateOf("") }
    var quantity by remember { mutableStateOf("1") }

    LaunchedEffect(Unit) { viewModel.loadCatalog() }

    Scaffold(topBar = { TopAppBar(title = { Text("New sale") }) }) { padding ->
        if (state.loading) {
            CircularProgressIndicator(modifier = Modifier.padding(padding).padding(24.dp))
            return@Scaffold
        }
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            PickerField(
                label = "Commerce",
                value = commerceId,
                options = state.commerces.map { it.id to (it.tradeName ?: it.legalName) },
                onSelect = { commerceId = it },
            )
            PickerField(
                label = "Payment",
                value = paymentMethod,
                options = PAYMENT_METHODS.map { it to it },
                onSelect = { paymentMethod = it },
            )
            PickerField(
                label = "Product",
                value = productId,
                options = state.products.map { it.id to "${it.name} (${it.sku})" },
                onSelect = { productId = it },
            )
            OutlinedTextField(
                value = quantity,
                onValueChange = { quantity = it },
                label = { Text("Quantity") },
                modifier = Modifier.fillMaxWidth(),
            )
            state.error?.let { Text(it) }
            Button(onClick = onBack, modifier = Modifier.fillMaxWidth()) { Text("Back") }
            Button(
                onClick = {
                    val qty = quantity.toIntOrNull() ?: return@Button
                    if (commerceId.isBlank() || paymentMethod.isBlank() || productId.isBlank()) return@Button
                    viewModel.createOfflineSale(commerceId, paymentMethod, listOf(productId to qty), onCreated)
                },
                enabled = !state.saving,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(if (state.saving) "Saving…" else "Confirm")
            }
        }
    }
}

@Composable
private fun PickerField(
    label: String,
    value: String,
    options: List<Pair<String, String>>,
    onSelect: (String) -> Unit,
) {
    var expanded by remember { mutableStateOf(false) }
    val labelText = options.firstOrNull { it.first == value }?.second ?: "Select $label"
    Column {
        Button(onClick = { expanded = true }, modifier = Modifier.fillMaxWidth()) {
            Text("$label: $labelText")
        }
        DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            options.forEach { (id, text) ->
                DropdownMenuItem(
                    text = { Text(text) },
                    onClick = {
                        onSelect(id)
                        expanded = false
                    },
                )
            }
        }
    }
}
