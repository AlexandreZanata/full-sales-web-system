package com.fullsales.seller.app

import androidx.compose.runtime.Composable
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.ui.SellerNavHost

@Composable
fun SellerRoot(container: SellerAppContainer) {
    SellerNavHost(container = container)
}
