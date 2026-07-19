package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CreditCard
import androidx.compose.material.icons.filled.Payments
import androidx.compose.material.icons.filled.QrCode2
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.theme.SellerCornerRadius
import com.fullsales.seller.app.ui.theme.PaymentCashAccent
import com.fullsales.seller.app.ui.theme.PaymentCreditAccent
import com.fullsales.seller.app.ui.theme.PaymentDebitAccent
import com.fullsales.seller.app.ui.theme.PaymentPixAccent
import com.fullsales.seller.shared.i18n.CreateSaleValidationError
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.sales.PAYMENT_METHODS

@Composable
internal fun PaymentMethodChips(
    selected: String,
    error: CreateSaleValidationError?,
    onSelect: (String) -> Unit,
) {
    val s = LocalSellerStrings.current
    FormSection(title = s.sales.paymentMethod) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            PAYMENT_METHODS.forEach { method ->
                PaymentMethodTile(
                    method = method,
                    selected = selected == method,
                    modifier = Modifier.weight(1f),
                    onSelect = { onSelect(method) },
                )
            }
        }
        error?.let {
            Text(
                SellerStrings.formatValidation(s, it),
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@Composable
private fun PaymentMethodTile(
    method: String,
    selected: Boolean,
    modifier: Modifier = Modifier,
    onSelect: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val label = SellerStrings.paymentMethod(s, method)
    val accent = paymentAccent(method)
    val borderColor = if (selected) accent else MaterialTheme.colorScheme.outlineVariant
    val contentColor = if (selected) accent else MaterialTheme.colorScheme.onSurfaceVariant
    Surface(
        modifier = modifier
            .height(88.dp)
            .selectableChipA11y(label, selected, s.a11y.selected)
            .clickable(role = Role.Button, onClick = onSelect),
        shape = RoundedCornerShape(SellerCornerRadius),
        color = MaterialTheme.colorScheme.surface,
        border = BorderStroke(if (selected) 2.dp else 1.dp, borderColor),
        tonalElevation = if (selected) 1.dp else 0.dp,
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 6.dp, vertical = 12.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Icon(paymentIcon(method), contentDescription = null, tint = contentColor, modifier = Modifier.size(26.dp))
            Text(
                label,
                style = MaterialTheme.typography.labelMedium,
                fontWeight = if (selected) FontWeight.SemiBold else FontWeight.Medium,
                color = contentColor,
                textAlign = TextAlign.Center,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
        }
    }
}

private fun paymentIcon(method: String): ImageVector = when (method) {
    "cash" -> Icons.Default.Payments
    "pix" -> Icons.Default.QrCode2
    "credit", "debit" -> Icons.Default.CreditCard
    else -> Icons.Default.Payments
}

private fun paymentAccent(method: String): Color = when (method) {
    "cash" -> PaymentCashAccent
    "pix" -> PaymentPixAccent
    "credit" -> PaymentCreditAccent
    "debit" -> PaymentDebitAccent
    else -> PaymentCashAccent
}
