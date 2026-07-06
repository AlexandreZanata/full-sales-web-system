package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.shared.i18n.CreateSaleValidationError
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.displayName
import com.fullsales.seller.shared.sales.PAYMENT_METHODS

@Composable
internal fun CommercePickerField(
    commerces: List<Commerce>,
    commerceId: String,
    error: CreateSaleValidationError?,
    onOpenPicker: () -> Unit,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    FormSection(title = s.sales.commerce) {
        androidx.compose.material3.Button(
            onClick = onOpenPicker,
            modifier = Modifier
                .fillMaxWidth()
                .defaultMinSize(minHeight = 48.dp),
        ) {
            Text(s.sales.browseCommerces)
        }
        CommerceQuickPickChips(
            commerces = commerces,
            commerceId = commerceId,
            onSelect = onSelect,
        )
        error?.let {
            Text(
                SellerStrings.formatValidation(s, it),
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun CommerceQuickPickChips(
    commerces: List<Commerce>,
    commerceId: String,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    FlowRow(
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        commerces.take(5).forEach { commerce ->
            FilterChip(
                selected = commerce.id == commerceId,
                onClick = { onSelect(commerce.id) },
                label = { Text(commerce.displayName()) },
                modifier = Modifier.selectableChipA11y(
                    commerce.displayName(),
                    commerce.id == commerceId,
                    s.a11y.selected,
                ),
            )
        }
    }
}

@Composable
internal fun PaymentMethodChips(
    selected: String,
    error: CreateSaleValidationError?,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    FormSection(title = s.sales.paymentMethod) {
        PaymentMethodChipRow(selected = selected, onSelect = onSelect)
        error?.let {
            Text(
                SellerStrings.formatValidation(s, it),
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun PaymentMethodChipRow(
    selected: String,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    FlowRow(
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        PAYMENT_METHODS.forEach { method ->
            FilterChip(
                selected = selected == method,
                onClick = { onSelect(method) },
                label = { Text(SellerStrings.paymentMethod(s, method)) },
                modifier = Modifier.selectableChipA11y(
                    SellerStrings.paymentMethod(s, method),
                    selected == method,
                    s.a11y.selected,
                ),
            )
        }
    }
}

@Composable
private fun FormSection(
    title: String,
    content: @Composable () -> Unit,
) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        Text(
            text = title,
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.SemiBold,
            color = MaterialTheme.colorScheme.onSurface,
        )
        content()
    }
}
