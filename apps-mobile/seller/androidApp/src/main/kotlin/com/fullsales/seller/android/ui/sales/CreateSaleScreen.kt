package com.fullsales.seller.android.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
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
import androidx.compose.ui.unit.dp
import com.fullsales.seller.shared.model.formatMoneyMinorUnits

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CreateSaleScreen(
    viewModel: CreateSaleViewModel,
    onBack: () -> Unit,
    onCreated: (String) -> Unit,
    onOpenCommercePicker: () -> Unit,
) {
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(state.snackbarMessage) {
        state.snackbarMessage?.let { message ->
            snackbarHostState.showSnackbar(message)
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
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            Text("New sale", style = MaterialTheme.typography.headlineSmall)
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
                Text(it, color = MaterialTheme.colorScheme.error, style = MaterialTheme.typography.bodySmall)
            }
            state.lines.forEachIndexed { index, line ->
                SaleLineCard(
                    line = line,
                    products = state.products,
                    stock = state.stockByProductId[line.productId],
                    quantityError = state.errors.lineErrors.getOrNull(index)?.quantityError,
                    onChange = { viewModel.updateLine(index, it) },
                    onRemove = { viewModel.removeLine(index) },
                    canRemove = state.lines.size > 1,
                )
            }
            TextButton(onClick = viewModel::addLine) {
                Icon(Icons.Default.Add, contentDescription = null)
                Text("Add line")
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
    Surface(shadowElevation = 8.dp) {
        Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text("Total", style = MaterialTheme.typography.titleMedium)
                Text(formatMoneyMinorUnits(totalMinor), style = MaterialTheme.typography.headlineSmall)
            }
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                TextButton(onClick = onBack, modifier = Modifier.weight(1f)) { Text("Back") }
                Button(
                    onClick = onSubmit,
                    enabled = !submitting,
                    modifier = Modifier.weight(1f),
                ) {
                    Text(if (submitting) "Saving…" else "Confirm sale")
                }
            }
        }
    }
}
