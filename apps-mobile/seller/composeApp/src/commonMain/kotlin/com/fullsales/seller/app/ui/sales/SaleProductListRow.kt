package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilledTonalIconButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.CreateSaleValidationError
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.TopSellingProduct
import com.fullsales.seller.shared.sales.CreateSaleLineInput

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun SaleProductListRow(
    line: CreateSaleLineInput,
    products: List<Product>,
    topSellingProducts: List<TopSellingProduct>,
    stock: Int?,
    quantityError: CreateSaleValidationError?,
    onChange: (CreateSaleLineInput) -> Unit,
    onRemove: () -> Unit,
    canRemove: Boolean,
) {
    val s = LocalSellerStrings.current
    val product = products.firstOrNull { it.id == line.productId }
    var editing by remember(line.productId) { mutableStateOf(line.productId.isBlank()) }
    val showPicker = editing || line.productId.isBlank()

    Surface(
        modifier = Modifier.fillMaxWidth(),
        shape = MaterialTheme.shapes.medium,
        color = MaterialTheme.colorScheme.surface,
        border = BorderStroke(1.dp, MaterialTheme.colorScheme.outlineVariant),
        tonalElevation = 2.dp,
    ) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
                verticalAlignment = Alignment.Top,
            ) {
                Column(
                    modifier = Modifier
                        .weight(1f)
                        .clickable(enabled = product != null) { editing = true },
                    verticalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    if (product != null && !showPicker) {
                        Text(
                            text = product.name,
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.SemiBold,
                            color = MaterialTheme.colorScheme.onSurface,
                        )
                        ProductSkuBadge(sku = product.sku)
                        stock?.let {
                            Text(
                                SellerStrings.stockBadge(s, it),
                                style = MaterialTheme.typography.bodyMedium,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                    } else {
                        Text(
                            text = s.sales.selectProduct,
                            style = MaterialTheme.typography.titleMedium,
                            color = MaterialTheme.colorScheme.primary,
                        )
                    }
                }
                if (canRemove) {
                    FilledTonalIconButton(
                        onClick = onRemove,
                        modifier = Modifier.defaultMinSize(minWidth = 48.dp, minHeight = 48.dp),
                    ) {
                        Icon(Icons.Default.Close, contentDescription = s.a11y.removeLine)
                    }
                }
            }
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = s.common.quantity,
                    style = MaterialTheme.typography.titleSmall,
                    fontWeight = FontWeight.Medium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.weight(1f, fill = false),
                )
                QuantityStepper(
                    value = line.quantityText,
                    onValueChange = { onChange(line.copy(quantityText = it)) },
                    isError = quantityError != null,
                )
            }
            quantityError?.let { err ->
                Text(
                    SellerStrings.formatValidation(s, err),
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
            if (showPicker) {
                ProductSearchPicker(
                    products = products,
                    topSellingProducts = topSellingProducts,
                    productId = line.productId,
                    searchQuery = line.productSearchQuery,
                    onSearchChange = { onChange(line.copy(productSearchQuery = it)) },
                    onSelect = {
                        onChange(line.copy(productId = it, productSearchQuery = ""))
                        editing = false
                    },
                    showSelectedLabel = false,
                )
            }
        }
    }
}

@Composable
private fun ProductSkuBadge(sku: String) {
    Surface(
        shape = MaterialTheme.shapes.small,
        color = MaterialTheme.colorScheme.primaryContainer,
    ) {
        Text(
            text = sku,
            modifier = Modifier.padding(horizontal = 10.dp, vertical = 4.dp),
            style = MaterialTheme.typography.labelLarge,
            fontWeight = FontWeight.Medium,
            color = MaterialTheme.colorScheme.onPrimaryContainer,
        )
    }
}
