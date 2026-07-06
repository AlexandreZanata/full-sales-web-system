package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
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
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.CreateSaleValidationError
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.shared.sales.CreateSaleLineInput
import com.fullsales.seller.shared.sales.PAYMENT_METHODS

@Composable
internal fun CommercePickerField(
    commerces: List<Commerce>,
    commerceId: String,
    error: CreateSaleValidationError?,
    onOpenPicker: () -> Unit,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Text(s.sales.commerce, style = MaterialTheme.typography.titleMedium)
        androidx.compose.material3.Button(
            onClick = onOpenPicker,
            modifier = Modifier
                .fillMaxWidth()
                .defaultMinSize(minHeight = 48.dp),
        ) {
            Text(s.sales.browseCommerces)
        }
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            commerces.take(5).forEach { commerce ->
                FilterChip(
                    selected = commerce.id == commerceId,
                    onClick = { onSelect(commerce.id) },
                    label = { Text(commerce.displayName()) },
                    modifier = Modifier.selectableChipA11y(
                        commerce.displayName(),
                        commerce.id == commerceId,
                        s.a11y.selected,
                    ),
                )
            }
        }
        error?.let {
            Text(
                SellerStrings.formatValidation(s, it),
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@Composable
internal fun PaymentMethodChips(
    selected: String,
    error: CreateSaleValidationError?,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Text(s.sales.paymentMethod, style = MaterialTheme.typography.titleMedium)
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            PAYMENT_METHODS.forEach { method ->
                FilterChip(
                    selected = selected == method,
                    onClick = { onSelect(method) },
                    label = { Text(SellerStrings.paymentMethod(s, method)) },
                    modifier = Modifier.selectableChipA11y(
                        SellerStrings.paymentMethod(s, method),
                        selected == method,
                        s.a11y.selected,
                    ),
                )
            }
        }
        error?.let {
            Text(
                SellerStrings.formatValidation(s, it),
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun SaleLineCard(
    line: CreateSaleLineInput,
    products: List<Product>,
    stock: Int?,
    quantityError: CreateSaleValidationError?,
    onChange: (CreateSaleLineInput) -> Unit,
    onRemove: () -> Unit,
    canRemove: Boolean,
) {
    val s = LocalSellerStrings.current
    Card(shape = MaterialTheme.shapes.medium) {
        Column(modifier = Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(s.sales.lineItem, style = MaterialTheme.typography.titleSmall, modifier = Modifier.weight(1f))
                if (canRemove) {
                    IconButton(onClick = onRemove) {
                        Icon(Icons.Default.Delete, contentDescription = s.a11y.removeLine)
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
                label = { Text(s.common.quantity) },
                isError = quantityError != null,
                supportingText = quantityError?.let { err ->
                    { Text(SellerStrings.formatValidation(s, err)) }
                },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true,
            )
            stock?.let {
                Text(
                    SellerStrings.stockBadge(s, it),
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
    val s = LocalSellerStrings.current
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        Text(s.sales.product, style = MaterialTheme.typography.labelLarge)
        products.take(8).forEach { product ->
            FilterChip(
                selected = product.id == productId,
                onClick = { onSelect(product.id) },
                label = { Text("${product.name} (${product.sku})", maxLines = 1) },
                modifier = Modifier.selectableChipA11y(
                    "${product.name} (${product.sku})",
                    product.id == productId,
                    s.a11y.selected,
                ),
            )
        }
    }
}
