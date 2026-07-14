package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.ime.bringIntoViewOnFocus
import com.fullsales.seller.shared.catalog.filterProductsForSalePickerSearch
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.TopSellingProduct
import com.fullsales.seller.shared.sales.filterProductsAvailableForBrowsing

@OptIn(ExperimentalLayoutApi::class)
@Composable
internal fun ProductSearchPicker(
    products: List<Product>,
    topSellingProducts: List<TopSellingProduct>,
    stockByProductId: Map<String, Int> = emptyMap(),
    productId: String,
    searchQuery: String,
    onSearchChange: (String) -> Unit,
    onSelect: (String) -> Unit,
    showSelectedLabel: Boolean = true,
) {
    val s = LocalSellerStrings.current
    val browseProducts = filterProductsAvailableForBrowsing(products, stockByProductId)
    val selected = browseProducts.firstOrNull { it.id == productId }
        ?: products.firstOrNull { it.id == productId }
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        if (showSelectedLabel) {
            Text(s.sales.product, style = MaterialTheme.typography.labelLarge)
            selected?.let {
                Text(
                    "${it.name} (${it.sku})",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.primary,
                )
            }
        }
        OutlinedTextField(
            value = searchQuery,
            onValueChange = onSearchChange,
            label = { Text(s.products.searchByNameOrSku) },
            modifier = Modifier
                .fillMaxWidth()
                .bringIntoViewOnFocus(),
            singleLine = true,
        )
        if (searchQuery.isBlank()) {
            if (topSellingProducts.isNotEmpty()) {
                TopSellingSection(
                    topSellingProducts = topSellingProducts,
                    productId = productId,
                    onSelect = onSelect,
                )
            }
            return@Column
        }
        SearchResultsSection(
            products = browseProducts,
            searchQuery = searchQuery,
            productId = productId,
            onSelect = onSelect,
        )
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun TopSellingSection(
    topSellingProducts: List<TopSellingProduct>,
    productId: String,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        Text(
            s.sales.topSelling,
            style = MaterialTheme.typography.labelMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        FlowRow(
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            topSellingProducts.forEach { item ->
                val label = "${item.name} (${item.sku})"
                FilterChip(
                    selected = productId == item.productId,
                    onClick = { onSelect(item.productId) },
                    label = { Text(label, maxLines = 1) },
                    modifier = Modifier.selectableChipA11y(
                        label,
                        productId == item.productId,
                        s.a11y.selected,
                    ),
                )
            }
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun SearchResultsSection(
    products: List<Product>,
    searchQuery: String,
    productId: String,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    val results = filterProductsForSalePickerSearch(products, searchQuery)
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        if (results.isEmpty()) {
            Text(
                s.sales.noProductResults,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.padding(top = 4.dp),
            )
            return
        }
        FlowRow(
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            results.forEach { product ->
                val label = "${product.name} (${product.sku})"
                FilterChip(
                    selected = productId == product.id,
                    onClick = { onSelect(product.id) },
                    label = { Text(label, maxLines = 1) },
                    modifier = Modifier.selectableChipA11y(
                        label,
                        productId == product.id,
                        s.a11y.selected,
                    ),
                )
            }
        }
    }
}
