package com.fullsales.seller.app.ui

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.fullsales.seller.app.ui.components.SellerEmptyState

@Composable
fun PlaceholderScreen(title: String, subtitle: String = "Coming in a later phase") {
    SellerEmptyState(
        title = title,
        message = subtitle,
        modifier = Modifier.fillMaxSize(),
    )
}
