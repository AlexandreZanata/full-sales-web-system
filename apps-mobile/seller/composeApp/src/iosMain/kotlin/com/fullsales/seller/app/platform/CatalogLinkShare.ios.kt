package com.fullsales.seller.app.platform

import platform.UIKit.UIActivityViewController
import platform.UIKit.UIApplication
import platform.UIKit.UIPasteboard
import platform.UIKit.UIViewController

actual object CatalogLinkShare {
    actual fun shareText(text: String, title: String) {
        val controller = UIActivityViewController(
            activityItems = listOf(text),
            applicationActivities = null,
        )
        val root = UIApplication.sharedApplication.keyWindow?.rootViewController
            ?: return
        val presenter = topPresenter(root)
        presenter.presentViewController(controller, animated = true, completion = null)
    }

    actual fun copyToClipboard(text: String, label: String) {
        UIPasteboard.generalPasteboard.string = text
    }

    private fun topPresenter(root: UIViewController): UIViewController {
        var current = root
        while (true) {
            val presented = current.presentedViewController ?: return current
            current = presented
        }
    }
}
