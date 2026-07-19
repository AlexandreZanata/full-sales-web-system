package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.theme.saleStatusPalette
import com.fullsales.seller.shared.i18n.SellerStrings
import com.fullsales.seller.shared.model.SaleDisplayStatus

@Composable
fun SaleStatusChip(status: SaleDisplayStatus, modifier: Modifier = Modifier) {
    val s = LocalSellerStrings.current
    val label = SellerStrings.saleStatus(s, status)
    val palette = saleStatusPalette(status)
    Surface(
        modifier = modifier,
        shape = RoundedCornerShape(50),
        color = palette.container,
        contentColor = palette.content,
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.labelSmall,
            fontWeight = FontWeight.SemiBold,
            color = palette.content,
            modifier = Modifier.padding(horizontal = 10.dp, vertical = 4.dp),
        )
    }
}
