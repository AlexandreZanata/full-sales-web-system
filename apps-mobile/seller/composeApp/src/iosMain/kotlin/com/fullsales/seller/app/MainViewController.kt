package com.fullsales.seller.app

import androidx.compose.runtime.remember
import androidx.compose.ui.window.ComposeUIViewController
import com.fullsales.seller.app.platform.ios.IosAppContainer
import platform.UIKit.UIViewController

fun MainViewController(): UIViewController = ComposeUIViewController {
    val container = remember { IosAppContainer() }
    SellerRoot(container = container)
}
