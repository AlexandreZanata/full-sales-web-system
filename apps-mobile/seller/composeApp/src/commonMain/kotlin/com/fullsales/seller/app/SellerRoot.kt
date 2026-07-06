package com.fullsales.seller.app

import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.fullsales.seller.app.platform.SellerAppContainer
import com.fullsales.seller.app.ui.SellerNavHost

@Composable
fun SellerRoot(container: SellerAppContainer) {
    Surface(modifier = Modifier) {
        SellerNavHost(container = container)
    }
}
