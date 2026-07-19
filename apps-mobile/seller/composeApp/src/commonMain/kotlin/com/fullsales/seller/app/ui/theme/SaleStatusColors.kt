package com.fullsales.seller.app.ui.theme

import androidx.compose.ui.graphics.Color
import com.fullsales.seller.shared.model.SaleDisplayStatus

data class SaleStatusPalette(
    val container: Color,
    val content: Color,
)

fun saleStatusPalette(status: SaleDisplayStatus): SaleStatusPalette = when (status) {
    SaleDisplayStatus.Pending -> SaleStatusPalette(
        container = Color(0xFFF59E0B),
        content = Color(0xFFFFFFFF),
    )
    SaleDisplayStatus.Confirmed -> SaleStatusPalette(
        container = Color(0xFF16A34A),
        content = Color(0xFFFFFFFF),
    )
    SaleDisplayStatus.Cancelled -> SaleStatusPalette(
        container = SellerError,
        content = SellerOnError,
    )
    SaleDisplayStatus.PendingSync -> SaleStatusPalette(
        container = SellerPrimary,
        content = SellerOnPrimary,
    )
    SaleDisplayStatus.SyncFailed -> SaleStatusPalette(
        container = Color(0xFFB91C1C),
        content = Color(0xFFFFFFFF),
    )
}
