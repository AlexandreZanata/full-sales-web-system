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
import com.fullsales.seller.android.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings

@Composable
fun ProductDetailScreen(
    productId: String,
    viewModel: ProductDetailViewModel,
    onAddToSale: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    LaunchedEffect(productId) { viewModel.load(productId) }

    when {
        state.loading -> CircularProgressIndicator(modifier = Modifier.padding(24.dp))
        state.errorCode != null -> Text(
            SellerStrings.productError(s, state.errorCode!!),
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
            stockAvailable = state.stockAvailable,
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
    stockAvailable: Int?,
    stockUnavailable: Boolean,
    imageUrl: String?,
    onAddToSale: () -> Unit,
) {
    val s = LocalSellerStrings.current
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        ProductHeroImage(imageUrl = imageUrl, contentDescription = name)
        Text(name, style = MaterialTheme.typography.headlineSmall)
        Text(
            SellerStrings.format(s.products.skuLabel, "value" to sku),
            style = MaterialTheme.typography.bodyMedium,
        )
        Text(priceLabel, style = MaterialTheme.typography.titleLarge)
        categoryName?.let {
            Text(
                SellerStrings.format(s.products.categoryLabel, "value" to it),
                style = MaterialTheme.typography.bodyMedium,
            )
        }
        unitOfMeasure?.let {
            Text(
                SellerStrings.format(s.products.unitLabel, "value" to it),
                style = MaterialTheme.typography.bodyMedium,
            )
        }
        Text(
            SellerStrings.stockBadge(s, stockAvailable),
            style = MaterialTheme.typography.labelLarge,
            color = if (stockUnavailable) {
                MaterialTheme.colorScheme.error
            } else {
                MaterialTheme.colorScheme.primary
            },
        )
        Text(
            if (active) s.common.active else s.common.inactive,
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
            Text(s.products.addToSale)
        }
    }
}
