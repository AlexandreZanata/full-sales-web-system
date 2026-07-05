package com.fullsales.seller.app.platform

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import com.fullsales.seller.shared.i18n.SellerLocale

actual class LocaleStore actual constructor() {
    private val prefs = appContext.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    actual fun read(): SellerLocale = SellerLocale.fromTag(prefs.getString(KEY_LOCALE, SellerLocale.DEFAULT.tag))

    actual fun write(locale: SellerLocale) {
        prefs.edit().putString(KEY_LOCALE, locale.tag).apply()
    }

    private companion object {
        const val PREFS_NAME = "seller_locale"
        const val KEY_LOCALE = "locale"
    }
}

lateinit var appContext: Context
    private set

fun initAndroidPlatform(context: Context) {
    appContext = context.applicationContext
}

actual fun createNetworkMonitor(): NetworkMonitor = AndroidNetworkMonitor()

actual fun isDebugBuild(): Boolean = com.fullsales.seller.composeapp.BuildConfig.IS_DEBUG

private class AndroidNetworkMonitor : NetworkMonitor {
    override fun isOnline(): Boolean {
        val connectivity = appContext.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        val network = connectivity.activeNetwork ?: return false
        val caps = connectivity.getNetworkCapabilities(network) ?: return false
        return caps.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }
}
