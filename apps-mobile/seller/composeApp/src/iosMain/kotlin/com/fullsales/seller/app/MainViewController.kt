package com.fullsales.seller.app

import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.window.ComposeUIViewController
import com.fullsales.seller.app.platform.ios.IosAppContainer
import kotlinx.coroutines.launch
import platform.Foundation.NSNotificationCenter
import platform.UIKit.UIApplicationDidBecomeActiveNotification
import platform.UIKit.UIViewController
import platform.darwin.NSObjectProtocol

fun MainViewController(): UIViewController = ComposeUIViewController {
    val container = remember { IosAppContainer() }
    val scope = rememberCoroutineScope()
    DisposableEffect(container) {
        val observer: NSObjectProtocol = NSNotificationCenter.defaultCenter.addObserverForName(
            name = UIApplicationDidBecomeActiveNotification,
            `object` = null,
            queue = null,
        ) {
            scope.launch { container.onAppResume() }
        }
        onDispose {
            NSNotificationCenter.defaultCenter.removeObserver(observer)
        }
    }
    SellerRoot(container = container)
}
