package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.a11y.TextSizePreset
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.DebouncedConnectivity
import com.fullsales.seller.shared.i18n.SellerLocale
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.StateFlow
import platform.Foundation.NSUserDefaults
import platform.Network.nw_path_get_status
import platform.Network.nw_path_monitor_create
import platform.Network.nw_path_monitor_set_queue
import platform.Network.nw_path_monitor_set_update_handler
import platform.Network.nw_path_monitor_start
import platform.Network.nw_path_status_satisfied
import platform.darwin.dispatch_get_main_queue

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

actual fun createNetworkMonitor(): NetworkMonitor = IosPathNetworkMonitor()

actual fun isDebugBuild(): Boolean = false

internal class IosPathNetworkMonitor : NetworkMonitor {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val gate = DebouncedConnectivity(scope)
    override val connectivity: StateFlow<ConnectivityState> = gate.state
    private val monitor = nw_path_monitor_create()

    init {
        nw_path_monitor_set_update_handler(monitor) { path ->
            gate.onValidatedChanged(nw_path_get_status(path) == nw_path_status_satisfied)
        }
        nw_path_monitor_set_queue(monitor, dispatch_get_main_queue())
        nw_path_monitor_start(monitor)
    }

    override fun reportPathReachable() {
        gate.promoteOnlineNow()
    }
}
