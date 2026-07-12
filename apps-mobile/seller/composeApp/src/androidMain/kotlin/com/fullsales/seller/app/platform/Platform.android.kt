package com.fullsales.seller.app.platform

import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import com.fullsales.seller.shared.a11y.TextSizePreset
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.DebouncedConnectivity
import com.fullsales.seller.shared.i18n.SellerLocale
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.StateFlow

actual class AccessibilityStore actual constructor() {
    private val prefs = appContext.getSharedPreferences(PREFS_A11Y, Context.MODE_PRIVATE)

    actual fun read(): TextSizePreset = TextSizePreset.fromTag(prefs.getString(KEY_TEXT_SIZE, null))

    actual fun write(preset: TextSizePreset) {
        prefs.edit().putString(KEY_TEXT_SIZE, preset.name).apply()
    }

    private companion object {
        const val PREFS_A11Y = "seller_a11y"
        const val KEY_TEXT_SIZE = "text_size_preset"
    }
}

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
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val gate = DebouncedConnectivity(scope)
    override val connectivity: StateFlow<ConnectivityState> = gate.state

    private val callback = object : ConnectivityManager.NetworkCallback() {
        override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) {
            gate.onValidatedChanged(caps.isValidatedInternet())
        }

        override fun onLost(network: Network) {
            gate.onValidatedChanged(false)
        }

        override fun onUnavailable() {
            gate.onValidatedChanged(false)
        }
    }

    init {
        val cm = appContext.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        val request = NetworkRequest.Builder()
            .addCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
            .build()
        cm.registerNetworkCallback(request, callback)
        gate.onValidatedChanged(cm.activeValidatedInternet())
    }
}

private fun ConnectivityManager.activeValidatedInternet(): Boolean {
    val network = activeNetwork ?: return false
    val caps = getNetworkCapabilities(network) ?: return false
    return caps.isValidatedInternet()
}

private fun NetworkCapabilities.isValidatedInternet(): Boolean =
    hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) &&
        hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED)
