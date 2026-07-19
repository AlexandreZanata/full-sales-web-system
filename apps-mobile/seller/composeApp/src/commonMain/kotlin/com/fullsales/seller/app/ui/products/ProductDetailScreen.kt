package com.fullsales.seller.app.ui.products

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ShoppingCart
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.SellerPrimaryButton
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.theme.SellerWarningColor
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
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        ProductHeroImage(imageUrl = imageUrl, contentDescription = name)
        SellerSurfaceCard(highlighted = true, contentPadding = false) {
            Column(
                modifier = Modifier.padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                Text(
                    name,
                    style = MaterialTheme.typography.headlineSmall,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onPrimaryContainer,
                )
                Text(
                    priceLabel,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onPrimaryContainer,
                )
                Text(
                    SellerStrings.format(s.products.skuLabel, "value" to sku),
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onPrimaryContainer,
                )
            }
        }
        SellerSurfaceCard(contentPadding = false) {
            Column(
                modifier = Modifier.padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp),
            ) {
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
                    if (stockUnavailable) {
                        SellerStrings.stockBackorderBadge(s)
                    } else {
                        SellerStrings.stockBadge(s, stockAvailable)
                    },
                    style = MaterialTheme.typography.labelLarge,
                    fontWeight = FontWeight.SemiBold,
                    color = if (stockUnavailable) {
                        SellerWarningColor
                    } else {
                        MaterialTheme.colorScheme.primary
                    },
                )
                if (stockUnavailable) {
                    Text(
                        SellerStrings.stockBackorderWarning(s),
                        style = MaterialTheme.typography.bodySmall,
                        color = SellerWarningColor,
                    )
                }
                Text(
                    if (active) s.common.active else s.common.inactive,
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                description?.takeIf { it.isNotBlank() }?.let {
                    Text(it, style = MaterialTheme.typography.bodyMedium)
                }
            }
        }
        SellerPrimaryButton(
            onClick = onAddToSale,
            leadingIcon = Icons.Default.ShoppingCart,
        ) {
            Text(s.products.addToSale)
        }
    }
}
