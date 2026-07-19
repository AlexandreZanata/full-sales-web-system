package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.SellerDangerButton
import com.fullsales.seller.app.ui.components.SellerPrimaryButton
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
            SellerDangerButton(
                onClick = onCancel,
                enabled = !acting,
                modifier = Modifier.weight(1f),
                leadingIcon = Icons.Default.Close,
                fillMaxWidth = false,
            ) {
                Text(s.sales.cancelShort)
            }
            SellerPrimaryButton(
                onClick = onConfirm,
                enabled = !acting,
                modifier = Modifier.weight(1f),
                leadingIcon = if (acting) null else Icons.Default.Check,
                fillMaxWidth = false,
            ) {
                if (acting) {
                    CircularProgressIndicator(modifier = Modifier.padding(4.dp).size(20.dp))
                } else {
                    Text(s.sales.confirmShort)
                }
            }
        }
    }
}
