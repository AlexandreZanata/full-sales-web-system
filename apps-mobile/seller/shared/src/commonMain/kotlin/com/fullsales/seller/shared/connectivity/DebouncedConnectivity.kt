package com.fullsales.seller.shared.connectivity

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * Flap-resistant connectivity gate (guia-cidade ADR-007 + OEM handoff).
 *
 * - Cold start is [ConnectivityState.Connecting] — never flash Offline before first OS read.
 * - Online after [onlineStableMs] of continuous reachability (timer does **not** restart on
 *   redundant `true` while already Connecting — Xiaomi CONNECTIVITY_CHANGE storms).
 * - Offline only after [offlineStableMs] of continuous unreachability (absorbs Wi‑Fi↔cell).
 * - [promoteOnlineNow] for proven HTTP path (device Offline ≠ API down).
 */
class DebouncedConnectivity(
    private val scope: CoroutineScope,
    private val onlineStableMs: Long = ONLINE_STABLE_MS,
    private val offlineStableMs: Long = OFFLINE_STABLE_MS,
    initial: ConnectivityState = ConnectivityState.Connecting,
) {
    private val _state = MutableStateFlow(initial)
    val state: StateFlow<ConnectivityState> = _state.asStateFlow()
    private var onlineJob: Job? = null
    private var offlineJob: Job? = null

    fun onValidatedChanged(validated: Boolean) {
        if (validated) {
            scheduleOnline()
        } else {
            scheduleOffline()
        }
    }

    /** Immediate Online when an API call already proved the path (auth, health, sync). */
    fun promoteOnlineNow() {
        onlineJob?.cancel()
        offlineJob?.cancel()
        onlineJob = null
        offlineJob = null
        _state.value = ConnectivityState.Online
    }

    private fun scheduleOnline() {
        offlineJob?.cancel()
        offlineJob = null
        if (_state.value == ConnectivityState.Online) return
        if (_state.value != ConnectivityState.Connecting) {
            _state.value = ConnectivityState.Connecting
        }
        if (onlineJob?.isActive == true) return
        onlineJob = scope.launch {
            delay(onlineStableMs)
            _state.value = ConnectivityState.Online
        }
    }

    private fun scheduleOffline() {
        onlineJob?.cancel()
        onlineJob = null
        if (_state.value == ConnectivityState.Offline) return
        if (offlineJob?.isActive == true) return
        offlineJob = scope.launch {
            delay(offlineStableMs)
            _state.value = ConnectivityState.Offline
        }
    }

    companion object {
        const val ONLINE_STABLE_MS: Long = 1_000L
        /** Absorb dual-radio / default-network swaps without Offline badge flash. */
        const val OFFLINE_STABLE_MS: Long = 1_200L
    }
}
