package com.fullsales.seller.shared.connectivity

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * Flap-resistant connectivity gate.
 * Offline is immediate; Online only after [onlineStableMs] of continuous validated reachability.
 */
class DebouncedConnectivity(
    private val scope: CoroutineScope,
    private val onlineStableMs: Long = ONLINE_STABLE_MS,
    initial: ConnectivityState = ConnectivityState.Offline,
) {
    private val _state = MutableStateFlow(initial)
    val state: StateFlow<ConnectivityState> = _state.asStateFlow()
    private var onlineJob: Job? = null

    fun onValidatedChanged(validated: Boolean) {
        if (!validated) {
            onlineJob?.cancel()
            onlineJob = null
            _state.value = ConnectivityState.Offline
            return
        }
        if (_state.value == ConnectivityState.Online) return
        if (_state.value != ConnectivityState.Connecting) {
            _state.value = ConnectivityState.Connecting
        }
        onlineJob?.cancel()
        onlineJob = scope.launch {
            delay(onlineStableMs)
            _state.value = ConnectivityState.Online
        }
    }

    companion object {
        const val ONLINE_STABLE_MS: Long = 2_000L
    }
}
