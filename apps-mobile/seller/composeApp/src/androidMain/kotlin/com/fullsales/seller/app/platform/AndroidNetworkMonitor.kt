package com.fullsales.seller.app.platform

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.provider.Settings
import android.util.Log
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.DebouncedConnectivity
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.StateFlow

/**
 * Device path observer (guia-cidade ADR-007 pattern).
 * OS capabilities drive the gate; successful HTTP uses [reportPathReachable] and must not
 * be conflated with API host errors (server banner stays separate).
 */
internal class AndroidNetworkMonitor : NetworkMonitor {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val gate = DebouncedConnectivity(scope)
    override val connectivity: StateFlow<ConnectivityState> = gate.state
    private val cm =
        appContext.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
    private val mainHandler = Handler(Looper.getMainLooper())
    @Volatile private var pathReachableUntilElapsedMs: Long = 0L

    private val callback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) = schedulePublish("available")
        override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) =
            schedulePublish("caps")
        override fun onLost(network: Network) = schedulePublish("lost")
        override fun onUnavailable() = schedulePublish("unavailable")
    }

    private val airplaneReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            schedulePublish(intent?.action ?: "airplane")
        }
    }

    init {
        cm.registerDefaultNetworkCallback(callback)
        val filter = IntentFilter(Intent.ACTION_AIRPLANE_MODE_CHANGED)
        if (Build.VERSION.SDK_INT >= 33) {
            appContext.registerReceiver(airplaneReceiver, filter, Context.RECEIVER_NOT_EXPORTED)
        } else {
            @Suppress("UnspecifiedRegisterReceiverFlag")
            appContext.registerReceiver(airplaneReceiver, filter)
        }
        schedulePublish("init")
    }

    override fun reportPathReachable() {
        pathReachableUntilElapsedMs = SystemClock.elapsedRealtime() + PATH_GRACE_MS
        Log.i(TAG, "reportPathReachable before=${gate.state.value}")
        gate.promoteOnlineNow()
        Log.i(TAG, "reportPathReachable after=${gate.state.value}")
    }

    private fun schedulePublish(reason: String) {
        mainHandler.post { publish(reason) }
    }

    private fun publish(reason: String) {
        val airplaneOn = Settings.Global.getInt(
            appContext.contentResolver,
            Settings.Global.AIRPLANE_MODE_ON,
            0,
        ) == 1
        val pathGrace = SystemClock.elapsedRealtime() < pathReachableUntilElapsedMs
        // MIUI can keep a stale validated Wi‑Fi agent briefly after airplane; trust the setting.
        // Path grace blocks OEM false-negatives after a proven HTTP round-trip.
        val online = !airplaneOn && (cm.anyUsableInternet() || pathGrace)
        val active = cm.activeNetwork
        val caps = active?.let { cm.getNetworkCapabilities(it) }
        val before = gate.state.value
        Log.i(
            TAG,
            "publish reason=$reason airplane=$airplaneOn online=$online pathGrace=$pathGrace " +
                "state=$before active=$active transports=${caps?.transportSummary()} " +
                "internet=${caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)} " +
                "validated=${caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED)}",
        )
        gate.onValidatedChanged(online)
        val after = gate.state.value
        if (after != before) {
            Log.i(TAG, "afterGate state=$after")
        }
    }

    private companion object {
        const val TAG = "SellerNet"
        const val PATH_GRACE_MS = 60_000L
    }
}

/** Prefer activeNetwork; fall back to any network (OEM quirks where activeNetwork is null). */
private fun ConnectivityManager.anyUsableInternet(): Boolean {
    val active = activeNetwork
    if (active != null) {
        val caps = getNetworkCapabilities(active)
        if (caps != null && caps.isUsableInternet()) return true
    }
    return allNetworks.any { network ->
        getNetworkCapabilities(network)?.isUsableInternet() == true
    }
}

private fun NetworkCapabilities.isUsableInternet(): Boolean {
    if (!hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)) return false
    if (hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED)) return true
    return hasTransport(NetworkCapabilities.TRANSPORT_WIFI) ||
        hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR) ||
        hasTransport(NetworkCapabilities.TRANSPORT_ETHERNET)
}

private fun NetworkCapabilities.transportSummary(): String = buildString {
    if (hasTransport(NetworkCapabilities.TRANSPORT_WIFI)) append("wifi ")
    if (hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR)) append("cell ")
    if (hasTransport(NetworkCapabilities.TRANSPORT_ETHERNET)) append("eth ")
}.ifBlank { "none" }
