package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.CreateSaleValidationError
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.TopSellingProduct
import com.fullsales.seller.shared.sales.CreateSaleLineErrors
import com.fullsales.seller.shared.sales.CreateSaleLineInput
import com.fullsales.seller.shared.sales.needsBackorderWarning
import com.fullsales.seller.shared.sales.visualStockRemaining

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun SaleProductListSection(
    lines: List<CreateSaleLineInput>,
    products: List<Product>,
    topSellingProducts: List<TopSellingProduct>,
    stockByProductId: Map<String, Int>,
    lineErrors: List<CreateSaleLineErrors>,
    linesError: CreateSaleValidationError?,
    mediaUrlResolver: MediaUrlResolver,
    onUpdateLine: (Int, CreateSaleLineInput) -> Unit,
    onRemoveLine: (Int) -> Unit,
    onAddLine: () -> Unit,
) {
    val s = LocalSellerStrings.current
    Card(
        shape = MaterialTheme.shapes.large,
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainerLow),
        border = BorderStroke(1.dp, MaterialTheme.colorScheme.outlineVariant),
        elevation = CardDefaults.cardElevation(defaultElevation = 1.dp),
    ) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            Text(
                text = s.sales.productList,
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.onSurface,
            )
            linesError?.let {
                Text(
                    SellerStrings.formatValidation(s, it),
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodyMedium,
                )
            }
            Column(verticalArrangement = Arrangement.spacedBy(12.dp)) {
                lines.forEachIndexed { index, line ->
                    SaleProductListRow(
                        line = line,
                        products = products,
                        topSellingProducts = topSellingProducts,
                        stockByProductId = stockByProductId,
                        stock = visualStockRemaining(
                            stockByProductId[line.productId],
                            lines,
                            line.productId,
                        ),
                        isBackorder = needsBackorderWarning(
                            line.productId,
                            lines,
                            stockByProductId,
                        ),
                        quantityError = lineErrors.getOrNull(index)?.quantityError,
                    mediaUrlResolver = mediaUrlResolver,
                    onChange = { onUpdateLine(index, it) },
                        onRemove = { onRemoveLine(index) },
                        canRemove = lines.size > 1,
                    )
                }
            }
            FilledTonalButton(
                onClick = onAddLine,
                modifier = Modifier
                    .fillMaxWidth()
                    .defaultMinSize(minHeight = 48.dp),
            ) {
                Icon(Icons.Default.Add, contentDescription = null)
                Text(s.sales.addLine, modifier = Modifier.padding(start = 8.dp))
            }
        }
    }
}
