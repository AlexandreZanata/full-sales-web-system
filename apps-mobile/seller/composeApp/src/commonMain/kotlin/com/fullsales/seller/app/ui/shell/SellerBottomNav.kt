package com.fullsales.seller.app.ui.shell

import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.navigationBars
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Receipt
import androidx.compose.material.icons.filled.Store
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.ripple
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.SellerRoutes
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings

/** Bottom nav: each tab owns a full 1/3 clickable column. */
@Composable
fun SellerBottomNav(
    currentRoute: String?,
    onNavigateSales: () -> Unit,
    onNavigateNewSale: () -> Unit,
    onNavigateCommerces: () -> Unit,
) {
    val s = LocalSellerStrings.current
    Surface(
        color = MaterialTheme.colorScheme.surface,
        tonalElevation = 0.dp,
        shadowElevation = 6.dp,
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .windowInsetsPadding(WindowInsets.navigationBars)
                .height(72.dp),
        ) {
            SellerNavSlot(
                selected = currentRoute == SellerRoutes.SALES,
                onClick = onNavigateSales,
                icon = Icons.Default.Receipt,
                label = s.nav.sales,
                contentDescription = s.a11y.sales,
            )
            SellerNavSlot(
                selected = currentRoute == SellerRoutes.SALES_NEW,
                onClick = onNavigateNewSale,
                icon = Icons.Default.Add,
                label = s.nav.newSale,
                contentDescription = s.a11y.newSale,
            )
            SellerNavSlot(
                selected = SellerRoutes.isCommerceTabSelected(currentRoute),
                onClick = onNavigateCommerces,
                icon = Icons.Default.Store,
                label = s.nav.commerces,
                contentDescription = s.a11y.commerces,
            )
        }
    }
}

@Composable
private fun RowScope.SellerNavSlot(
    selected: Boolean,
    onClick: () -> Unit,
    icon: ImageVector,
    label: String,
    contentDescription: String,
) {
    val interaction = remember { MutableInteractionSource() }
    val contentColor = if (selected) {
        MaterialTheme.colorScheme.primary
    } else {
        MaterialTheme.colorScheme.onSurfaceVariant
    }
    Column(
        modifier = Modifier
            .weight(1f)
            .fillMaxHeight()
            .semantics {
                this.contentDescription = contentDescription
                this.selected = selected
            }
            .clickable(
                interactionSource = interaction,
                indication = ripple(bounded = true),
                role = Role.Tab,
                onClick = onClick,
            )
            .padding(horizontal = 4.dp, vertical = 8.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center,
    ) {
        Surface(
            shape = RoundedCornerShape(16.dp),
            color = if (selected) {
                MaterialTheme.colorScheme.primary
            } else {
                MaterialTheme.colorScheme.surface
            },
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                tint = if (selected) {
                    MaterialTheme.colorScheme.onPrimary
                } else {
                    contentColor
                },
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 6.dp),
            )
        }
        Text(
            text = label,
            style = MaterialTheme.typography.labelMedium,
            fontWeight = if (selected) FontWeight.SemiBold else FontWeight.Medium,
            color = contentColor,
            textAlign = TextAlign.Center,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
            modifier = Modifier.padding(top = 4.dp),
        )
    }
}
