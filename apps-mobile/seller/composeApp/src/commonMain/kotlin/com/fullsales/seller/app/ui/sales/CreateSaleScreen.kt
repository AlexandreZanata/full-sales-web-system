package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
import com.fullsales.seller.app.ui.shell.SellerStickyBottomBar
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.platform.MediaUrlResolver
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.formatMoneyMinorUnits

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CreateSaleScreen(
    viewModel: CreateSaleViewModel,
    mediaUrlResolver: MediaUrlResolver,
    onBack: () -> Unit,
    onCreated: (String) -> Unit,
    onOpenCommercePicker: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            snackbarHostState.showSnackbar(SellerStrings.createSaleError(s, code))
            viewModel.clearSnackbar()
        }
    }
    NestedScreenScaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) },
        bottomBar = {
            CreateSaleBottomBar(
                totalMinor = state.totalMinor,
                submitting = state.submitting,
                onBack = onBack,
                onSubmit = { viewModel.submit(onCreated) },
            )
        },
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .verticalScroll(rememberScrollState())
                .padding(horizontal = 16.dp)
                .padding(top = 4.dp, bottom = 12.dp)
                .padding(bottom = 16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    s.sales.new,
                    style = MaterialTheme.typography.headlineSmall,
                    modifier = Modifier
                        .weight(1f, fill = false)
                        .screenTitle(),
                )
                TextButton(
                    onClick = viewModel::clearForm,
                    enabled = state.hasPersistedContent && !state.submitting,
                    modifier = Modifier.defaultMinSize(minHeight = 48.dp),
                ) {
                    Text(s.sales.clearForm)
                }
            }
            CommercePickerField(
                commerces = state.commerces,
                commerceId = state.commerceId,
                error = state.errors.commerceError,
                onOpenPicker = onOpenCommercePicker,
                onSelect = viewModel::setCommerceId,
            )
            PaymentMethodChips(
                selected = state.paymentMethod,
                error = state.errors.paymentError,
                onSelect = viewModel::setPaymentMethod,
            )
            SaleProductListSection(
                lines = state.lines,
                products = state.products,
                topSellingProducts = state.topSellingProducts,
                stockByProductId = state.stockByProductId,
                lineErrors = state.errors.lineErrors,
                linesError = state.errors.linesError,
                mediaUrlResolver = mediaUrlResolver,
                onUpdateLine = viewModel::updateLine,
                onRemoveLine = viewModel::removeLine,
                onAddLine = viewModel::addLine,
            )
        }
    }
}

@Composable
private fun CreateSaleBottomBar(
    totalMinor: Long,
    submitting: Boolean,
    onBack: () -> Unit,
    onSubmit: () -> Unit,
) {
    val s = LocalSellerStrings.current
    SellerStickyBottomBar {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(
                s.common.total,
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Text(
                formatMoneyMinorUnits(totalMinor),
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.SemiBold,
                color = MaterialTheme.colorScheme.primary,
            )
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            OutlinedButton(
                onClick = onBack,
                modifier = Modifier
                    .weight(1f)
                    .defaultMinSize(minHeight = 48.dp),
            ) {
                Text(s.common.back)
            }
            Button(
                onClick = onSubmit,
                enabled = !submitting,
                modifier = Modifier
                    .weight(1f)
                    .defaultMinSize(minHeight = 48.dp),
            ) {
                Text(if (submitting) s.common.saving else s.sales.confirmShort)
            }
        }
    }
}
