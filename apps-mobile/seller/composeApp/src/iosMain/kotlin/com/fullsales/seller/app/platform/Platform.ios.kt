package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.a11y.TextSizePreset
import com.fullsales.seller.shared.i18n.SellerLocale
import platform.Foundation.NSUserDefaults

actual class AccessibilityStore actual constructor() {
    private val defaults = NSUserDefaults.standardUserDefaults

    actual fun read(): TextSizePreset = TextSizePreset.fromTag(defaults.stringForKey(KEY_TEXT_SIZE))

    actual fun write(preset: TextSizePreset) {
        defaults.setObject(preset.name, KEY_TEXT_SIZE)
    }

    private companion object {
        const val KEY_TEXT_SIZE = "seller_text_size_preset"
    }
}

actual class LocaleStore actual constructor() {
    private val defaults = NSUserDefaults.standardUserDefaults

    actual fun read(): SellerLocale =
        SellerLocale.fromTag(defaults.stringForKey(KEY_LOCALE))

    actual fun write(locale: SellerLocale) {
        defaults.setObject(locale.tag, KEY_LOCALE)
    }

    private companion object {
        const val KEY_LOCALE = "seller_locale"
    }
}

actual fun createNetworkMonitor(): NetworkMonitor = object : NetworkMonitor {
    override fun isOnline(): Boolean = true
}

actual fun isDebugBuild(): Boolean = false
