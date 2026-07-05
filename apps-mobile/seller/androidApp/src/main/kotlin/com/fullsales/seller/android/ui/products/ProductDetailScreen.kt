package com.fullsales.seller.android.ui.products

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun ProductDetailScreen(
    productId: String,
    viewModel: ProductDetailViewModel,
    onAddToSale: (String) -> Unit,
) {
    val state by viewModel.state.collectAsState()
    LaunchedEffect(productId) { viewModel.load(productId) }

    when {
        state.loading -> CircularProgressIndicator(modifier = Modifier.padding(24.dp))
        state.error != null -> Text(
            state.error!!,
            color = MaterialTheme.colorScheme.error,
            modifier = Modifier.padding(16.dp),
        )
        state.product != null -> ProductDetailContent(
            name = state.product!!.name,
            sku = state.product!!.sku,
            priceLabel = state.priceLabel.orEmpty(),
            categoryName = state.product!!.categoryName,
            unitOfMeasure = state.product!!.unitOfMeasure,
            description = state.product!!.description,
            active = state.product!!.active,
            stockLabel = state.stockLabel,
            stockUnavailable = state.stockUnavailable,
            imageUrl = state.imageUrl,
            onAddToSale = { onAddToSale(productId) },
        )
    }
}

@Composable
private fun ProductDetailContent(
    name: String,
    sku: String,
    priceLabel: String,
    categoryName: String?,
    unitOfMeasure: String?,
    description: String?,
    active: Boolean,
    stockLabel: String,
    stockUnavailable: Boolean,
    imageUrl: String?,
    onAddToSale: () -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        ProductHeroImage(imageUrl = imageUrl, contentDescription = name)
        Text(name, style = MaterialTheme.typography.headlineSmall)
        Text("SKU: $sku", style = MaterialTheme.typography.bodyMedium)
        Text(priceLabel, style = MaterialTheme.typography.titleLarge)
        categoryName?.let { Text("Category: $it", style = MaterialTheme.typography.bodyMedium) }
        unitOfMeasure?.let { Text("Unit: $it", style = MaterialTheme.typography.bodyMedium) }
        Text(
            stockLabel,
            style = MaterialTheme.typography.labelLarge,
            color = if (stockUnavailable) {
                MaterialTheme.colorScheme.error
            } else {
                MaterialTheme.colorScheme.primary
            },
        )
        Text(
            if (active) "Active" else "Inactive",
            style = MaterialTheme.typography.labelMedium,
        )
        description?.takeIf { it.isNotBlank() }?.let {
            Text(it, style = MaterialTheme.typography.bodyMedium)
        }
        Button(
            onClick = onAddToSale,
            enabled = !stockUnavailable,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text("Add to sale")
        }
    }
}
