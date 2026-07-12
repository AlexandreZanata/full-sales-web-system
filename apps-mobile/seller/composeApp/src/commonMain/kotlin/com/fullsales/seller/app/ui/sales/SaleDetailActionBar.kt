package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.shell.SellerStickyBottomBar
import com.fullsales.seller.shared.model.formatMoneyMinorUnits

@Composable
internal fun SaleDetailActionBar(
    showActions: Boolean,
    acting: Boolean,
    totalMinor: Long,
    currency: String,
    onConfirm: () -> Unit,
    onCancel: () -> Unit,
) {
    if (!showActions) return
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
                formatMoneyMinorUnits(totalMinor, currency),
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.primary,
            )
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            OutlinedButton(
                onClick = onCancel,
                enabled = !acting,
                modifier = Modifier
                    .weight(1f)
                    .defaultMinSize(minHeight = 48.dp),
                colors = androidx.compose.material3.ButtonDefaults.outlinedButtonColors(
                    contentColor = MaterialTheme.colorScheme.error,
                ),
            ) {
                Text(s.sales.cancelShort)
            }
            Button(
                onClick = onConfirm,
                enabled = !acting,
                modifier = Modifier
                    .weight(1f)
                    .defaultMinSize(minHeight = 48.dp),
            ) {
                if (acting) {
                    CircularProgressIndicator(modifier = Modifier.padding(4.dp))
                } else {
                    Text(s.sales.confirm)
                }
            }
        }
    }
}
