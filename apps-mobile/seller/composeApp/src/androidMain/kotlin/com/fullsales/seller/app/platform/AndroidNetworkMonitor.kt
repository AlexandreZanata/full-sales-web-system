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
import android.provider.Settings
import android.util.Log
import com.fullsales.seller.shared.connectivity.ConnectivityState
import com.fullsales.seller.shared.connectivity.DebouncedConnectivity
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.StateFlow

internal class AndroidNetworkMonitor : NetworkMonitor {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val gate = DebouncedConnectivity(scope)
    override val connectivity: StateFlow<ConnectivityState> = gate.state
    private val cm =
        appContext.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
    private val mainHandler = Handler(Looper.getMainLooper())

    private val callback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) = schedulePublish("available")
        override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) =
            schedulePublish("caps")
        override fun onLost(network: Network) = schedulePublish("lost")
        override fun onUnavailable() = schedulePublish("unavailable")
    }

    private val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            schedulePublish(intent?.action ?: "broadcast")
        }
    }

    init {
        cm.registerDefaultNetworkCallback(callback)
        val filter = IntentFilter().apply {
            @Suppress("DEPRECATION")
            addAction(ConnectivityManager.CONNECTIVITY_ACTION)
            addAction(Intent.ACTION_AIRPLANE_MODE_CHANGED)
        }
        if (Build.VERSION.SDK_INT >= 33) {
            appContext.registerReceiver(broadcastReceiver, filter, Context.RECEIVER_NOT_EXPORTED)
        } else {
            @Suppress("UnspecifiedRegisterReceiverFlag")
            appContext.registerReceiver(broadcastReceiver, filter)
        }
        schedulePublish("init")
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
        // MIUI can keep a stale validated Wi‑Fi agent briefly after airplane; trust the setting.
        val online = !airplaneOn && cm.activeUsableInternet()
        val active = cm.activeNetwork
        val caps = active?.let { cm.getNetworkCapabilities(it) }
        Log.i(
            TAG,
            "publish reason=$reason airplane=$airplaneOn online=$online state=${gate.state.value} " +
                "active=$active transports=${caps?.transportSummary()} " +
                "internet=${caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)} " +
                "validated=${caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED)}",
        )
        gate.onValidatedChanged(online)
        Log.i(TAG, "afterGate state=${gate.state.value}")
    }

    private companion object {
        const val TAG = "SellerNet"
    }
}

private fun ConnectivityManager.activeUsableInternet(): Boolean {
    val network = activeNetwork ?: return false
    val caps = getNetworkCapabilities(network) ?: return false
    return caps.isUsableInternet()
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
