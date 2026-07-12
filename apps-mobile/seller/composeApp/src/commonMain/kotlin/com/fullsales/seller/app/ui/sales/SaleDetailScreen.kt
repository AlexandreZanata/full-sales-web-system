package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.ui.components.SellerEmptyState
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.sales.SaleDetailModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SaleDetailScreen(
    saleId: String,
    viewModel: SaleDetailViewModel,
    mediaUrlResolver: MediaUrlResolver,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(saleId) { viewModel.load(saleId) }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            snackbarHostState.showSnackbar(SellerStrings.saleActionError(s, code))
            viewModel.clearSnackbar()
        }
    }
    val detail = state.detail
    NestedScreenScaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) },
        bottomBar = {
            if (detail != null) {
                SaleDetailActionBar(
                    showActions = detail.showActions,
                    acting = state.acting,
                    totalMinor = detail.totalAmountMinor.toLong(),
                    currency = detail.totalCurrency,
                    onConfirm = viewModel::confirm,
                    onCancel = viewModel::cancel,
                )
            }
        },
    ) { padding ->
        when {
            state.loading -> CircularProgressIndicator(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding)
                    .padding(24.dp),
            )
            state.errorCode != null -> SellerEmptyState(
                title = s.sales.loadErrorTitle,
                message = SellerStrings.saleActionError(s, state.errorCode!!),
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding),
            )
            detail != null -> SaleDetailBody(
                detail = detail,
                stockByProductId = state.stockByProductId,
                hasBottomActionBar = detail.showActions,
                mediaUrlResolver = mediaUrlResolver,
                modifier = Modifier.padding(padding),
            )
        }
    }
}

@Composable
private fun SaleDetailBody(
    detail: SaleDetailModel,
    stockByProductId: Map<String, Int>,
    hasBottomActionBar: Boolean,
    mediaUrlResolver: MediaUrlResolver,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = 16.dp, vertical = 8.dp)
            .padding(bottom = 16.dp)
            .then(if (hasBottomActionBar) Modifier else Modifier.navigationBarsPadding()),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        SaleDetailHeader(detail = detail)
        SaleDetailSummaryCard(detail = detail)
        SaleDetailMetaCard(detail = detail)
        SaleDetailItemsCard(
            items = detail.items,
            stockByProductId = stockByProductId,
            showBackorderHints = detail.showActions,
            mediaUrlResolver = mediaUrlResolver,
        )
    }
}
