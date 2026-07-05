package com.fullsales.seller.android.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.shared.sales.CreateSaleLineInput
import com.fullsales.seller.shared.sales.PAYMENT_METHODS
import com.fullsales.seller.shared.sales.paymentMethodLabel

@Composable
internal fun CommercePickerField(
    commerces: List<Commerce>,
    commerceId: String,
    error: String?,
    onOpenPicker: () -> Unit,
    onSelect: (String) -> Unit,
) {
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Text("Commerce", style = MaterialTheme.typography.titleMedium)
        androidx.compose.material3.Button(onClick = onOpenPicker, modifier = Modifier.fillMaxWidth()) {
            Text("Browse commerces")
        }
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            commerces.take(5).forEach { commerce ->
                FilterChip(
                    selected = commerce.id == commerceId,
                    onClick = { onSelect(commerce.id) },
                    label = { Text(commerce.displayName()) },
                )
            }
        }
        error?.let {
            Text(it, color = MaterialTheme.colorScheme.error, style = MaterialTheme.typography.bodySmall)
        }
    }
}

@Composable
internal fun PaymentMethodChips(
    selected: String,
    error: String?,
    onSelect: (String) -> Unit,
) {
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Text("Payment method", style = MaterialTheme.typography.titleMedium)
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            PAYMENT_METHODS.forEach { method ->
                FilterChip(
                    selected = selected == method,
                    onClick = { onSelect(method) },
                    label = { Text(paymentMethodLabel(method)) },
                )
            }
        }
        error?.let {
            Text(it, color = MaterialTheme.colorScheme.error, style = MaterialTheme.typography.bodySmall)
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun SaleLineCard(
    line: CreateSaleLineInput,
    products: List<Product>,
    stock: Int?,
    quantityError: String?,
    onChange: (CreateSaleLineInput) -> Unit,
    onRemove: () -> Unit,
    canRemove: Boolean,
) {
    Card(shape = MaterialTheme.shapes.medium) {
        Column(modifier = Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text("Line item", style = MaterialTheme.typography.titleSmall, modifier = Modifier.weight(1f))
                if (canRemove) {
                    IconButton(onClick = onRemove) {
                        Icon(Icons.Default.Delete, contentDescription = "Remove line")
                    }
                }
            }
            ProductPickerChips(
                products = products,
                productId = line.productId,
                onSelect = { onChange(line.copy(productId = it)) },
            )
            OutlinedTextField(
                value = line.quantityText,
                onValueChange = { onChange(line.copy(quantityText = it)) },
                label = { Text("Quantity") },
                isError = quantityError != null,
                supportingText = quantityError?.let { { Text(it) } },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true,
            )
            stock?.let {
                Text(
                    "Available: $it",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun ProductPickerChips(
    products: List<Product>,
    productId: String,
    onSelect: (String) -> Unit,
) {
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        Text("Product", style = MaterialTheme.typography.labelLarge)
        products.take(8).forEach { product ->
            FilterChip(
                selected = product.id == productId,
                onClick = { onSelect(product.id) },
                label = { Text("${product.name} (${product.sku})", maxLines = 1) },
            )
        }
    }
}
