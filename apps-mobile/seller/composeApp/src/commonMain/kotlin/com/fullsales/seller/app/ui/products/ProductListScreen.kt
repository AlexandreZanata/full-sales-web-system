package com.fullsales.seller.app.ui.products

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.ui.a11y.listItemSummary
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.components.SellerEmptyState
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.formatProductPrice
import com.fullsales.seller.shared.ui.ListEmptyDomain
import com.fullsales.seller.shared.ui.ListEmptyReason
import com.fullsales.seller.shared.ui.listEmptyCopy
import com.fullsales.seller.shared.ui.listSnackbarMessage

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProductListScreen(
    viewModel: ProductViewModel,
    mediaUrlResolver: MediaUrlResolver,
    onProductClick: (String) -> Unit,
    title: String? = null,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            snackbarHostState.showSnackbar(listSnackbarMessage(s, code))
            viewModel.clearSnackbar()
        }
    }
    NestedScreenScaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) },
    ) { padding ->
        PullToRefreshBox(
            isRefreshing = state.refreshing,
            onRefresh = { viewModel.refresh() },
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .navigationBarsPadding()
                .semantics { contentDescription = s.a11y.pullToRefresh },
        ) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                Text(
                    title ?: s.products.title,
                    style = MaterialTheme.typography.headlineSmall,
                    modifier = Modifier.screenTitle(),
                )
                OutlinedTextField(
                    value = state.searchQuery,
                    onValueChange = viewModel::setSearchQuery,
                    label = { Text(s.products.searchByNameOrSku) },
                    modifier = Modifier.fillMaxWidth(),
                    singleLine = true,
                )
                when {
                    state.items.isEmpty() &&
                        state.emptyReason != null &&
                        state.emptyReason != ListEmptyReason.RefreshFailedKeepCache -> {
                        val copy = listEmptyCopy(s, state.emptyReason!!, ListEmptyDomain.Products)
                        SellerEmptyState(
                            title = copy.title,
                            message = copy.message,
                            modifier = Modifier
                                .fillMaxSize()
                                .semantics { contentDescription = copy.announcement },
                        )
                    }
                    state.isFilterEmpty -> Text(s.products.empty, style = MaterialTheme.typography.bodyLarge)
                    else -> LazyColumn(contentPadding = PaddingValues(bottom = 16.dp)) {
                        items(state.filtered, key = { it.id }) { product ->
                            ProductRow(
                                product = product,
                                mediaUrlResolver = mediaUrlResolver,
                                onClick = { onProductClick(product.id) },
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun ProductRow(
    product: Product,
    mediaUrlResolver: MediaUrlResolver,
    onClick: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val price = formatProductPrice(product.priceAmount, product.priceCurrency)
    val summary = SellerStrings.productListItem(s, product.name, product.sku, price)
    var imageUrl by remember(product.id) { mutableStateOf(product.primaryImageUrl) }
    LaunchedEffect(product.id, product.primaryImageUrl, product.primaryImageFileId) {
        imageUrl = mediaUrlResolver.resolveImageUrl(product.primaryImageUrl, product.primaryImageFileId)
    }
    SellerSurfaceCard(
        modifier = Modifier
            .padding(vertical = 4.dp)
            .listItemSummary(summary),
        onClick = onClick,
        contentPadding = false,
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(14.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            ProductThumbnail(
                imageUrl = imageUrl,
                contentDescription = product.name,
            )
            Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(2.dp)) {
                Text(
                    product.name,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    product.sku,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                product.categoryName?.let {
                    Text(
                        it,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
            Text(
                formatProductPrice(product.priceAmount, product.priceCurrency),
                style = MaterialTheme.typography.titleSmall,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.primary,
            )
        }
    }
}
