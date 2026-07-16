package com.fullsales.field.android.connectivity

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.os.Build
import android.provider.Settings
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/** Thin device connectivity for Field offline UX (Phase 18F). */
class FieldNetworkMonitor(context: Context) {
    private val appContext = context.applicationContext
    private val cm = appContext.getSystemService(ConnectivityManager::class.java)
    private val _online = MutableStateFlow(readOnline())
    val online: StateFlow<Boolean> = _online.asStateFlow()

    private val callback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) = publish()
        override fun onLost(network: Network) = publish()
        override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) = publish()
    }

    private val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) = publish()
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
        publish()
    }

    fun isOnline(): Boolean = _online.value

    private fun publish() {
        _online.value = readOnline()
    }

    private fun readOnline(): Boolean {
        val airplaneOn = Settings.Global.getInt(
            appContext.contentResolver,
            Settings.Global.AIRPLANE_MODE_ON,
            0,
        ) == 1
        if (airplaneOn) return false
        val network = cm.activeNetwork ?: return false
        val caps = cm.getNetworkCapabilities(network) ?: return false
        return caps.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }
}
