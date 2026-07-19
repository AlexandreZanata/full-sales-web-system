package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.IconButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.theme.SellerWarningColor
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
    stockByProductId: Map<String, Int>,
    stock: Int?,
    isBackorder: Boolean,
    quantityError: CreateSaleValidationError?,
    mediaUrlResolver: MediaUrlResolver,
    onChange: (CreateSaleLineInput) -> Unit,
    onRemove: () -> Unit,
    canRemove: Boolean,
) {
    val s = LocalSellerStrings.current
    val product = products.firstOrNull { it.id == line.productId }
    val hasProduct = product != null
    val showRemove = hasProduct || canRemove

    SellerSurfaceCard(contentPadding = false) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp),
        ) {
            ProductRowHeader(
                product = product,
                stock = stock,
                isBackorder = isBackorder,
                showRemove = showRemove,
                mediaUrlResolver = mediaUrlResolver,
                selectProductLabel = s.sales.selectProduct,
                onRemove = onRemove,
                removeContentDescription = s.a11y.removeLine,
            )
            if (hasProduct) {
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
                    )
                    QuantityStepper(
                        value = line.quantityText,
                        onValueChange = { onChange(line.copy(quantityText = it)) },
                        isError = quantityError != null,
                        isWarning = isBackorder,
                    )
                }
            }
            if (isBackorder) {
                Text(
                    SellerStrings.stockBackorderWarning(s),
                    color = SellerWarningColor,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
            quantityError?.let { err ->
                Text(
                    SellerStrings.formatValidation(s, err),
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
            if (!hasProduct) {
                ProductSearchPicker(
                    products = products,
                    topSellingProducts = topSellingProducts,
                    stockByProductId = stockByProductId,
                    productId = line.productId,
                    searchQuery = line.productSearchQuery,
                    onSearchChange = { onChange(line.copy(productSearchQuery = it)) },
                    onSelect = { onChange(line.copy(productId = it, productSearchQuery = "")) },
                    showSelectedLabel = false,
                )
            }
        }
    }
}

@Composable
private fun ProductRowHeader(
    product: Product?,
    stock: Int?,
    isBackorder: Boolean,
    showRemove: Boolean,
    mediaUrlResolver: MediaUrlResolver,
    selectProductLabel: String,
    onRemove: () -> Unit,
    removeContentDescription: String,
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.Top,
    ) {
        if (product != null) {
            SaleLineProductImage(
                product = product,
                mediaUrlResolver = mediaUrlResolver,
                contentDescription = product.name,
            )
        }
        Column(
            modifier = Modifier.weight(1f),
            verticalArrangement = Arrangement.spacedBy(6.dp),
        ) {
            if (product != null) {
                Text(
                    text = product.name,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onSurface,
                )
                ProductSkuBadge(sku = product.sku)
                if (isBackorder) BackorderBadge() else stock?.let { AvailableStockLabel(it) }
            } else {
                Text(
                    text = selectProductLabel,
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.primary,
                )
            }
        }
        if (showRemove) {
            IconButton(
                onClick = onRemove,
                modifier = Modifier.size(40.dp),
                colors = IconButtonDefaults.iconButtonColors(
                    containerColor = MaterialTheme.colorScheme.errorContainer,
                    contentColor = MaterialTheme.colorScheme.error,
                ),
            ) {
                Icon(Icons.Default.Close, contentDescription = removeContentDescription)
            }
        }
    }
}
