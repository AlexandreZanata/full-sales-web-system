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
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.formatMoneyMinorUnits

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CreateSaleScreen(
    viewModel: CreateSaleViewModel,
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
    Scaffold(
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
                .padding(horizontal = 16.dp, vertical = 16.dp)
                .padding(bottom = 24.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            Text(
                s.sales.new,
                style = MaterialTheme.typography.headlineSmall,
                modifier = Modifier.screenTitle(),
            )
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
            state.errors.linesError?.let {
                Text(
                    SellerStrings.formatValidation(s, it),
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
            state.lines.forEachIndexed { index, line ->
                SaleLineCard(
                    line = line,
                    products = state.products,
                    topSellingProducts = state.topSellingProducts,
                    stock = state.stockByProductId[line.productId],
                    quantityError = state.errors.lineErrors.getOrNull(index)?.quantityError,
                    onChange = { viewModel.updateLine(index, it) },
                    onRemove = { viewModel.removeLine(index) },
                    canRemove = state.lines.size > 1,
                )
            }
            TextButton(
                onClick = viewModel::addLine,
                modifier = Modifier.defaultMinSize(minHeight = 48.dp),
            ) {
                Icon(
                    Icons.Default.Add,
                    contentDescription = null,
                    modifier = Modifier.clearAndSetSemantics { },
                )
                Text(s.sales.addLine)
            }
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
    Surface(shadowElevation = 8.dp) {
        Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(s.common.total, style = MaterialTheme.typography.titleMedium)
                Text(formatMoneyMinorUnits(totalMinor), style = MaterialTheme.typography.headlineSmall)
            }
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
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
}
