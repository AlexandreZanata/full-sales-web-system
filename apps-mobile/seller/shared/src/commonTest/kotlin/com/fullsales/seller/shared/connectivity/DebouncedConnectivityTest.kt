package com.fullsales.seller.shared.connectivity

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.advanceTimeBy
import kotlinx.coroutines.test.runCurrent
import kotlinx.coroutines.test.runTest

/**
 * Contract: cold start Connecting (no Offline flash); Online after stable window without
 * restarting on redundant true; Offline only after stable unreachability; promoteOnlineNow.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class DebouncedConnectivityTest {
    @Test
    fun given_cold_start_when_validated_then_online_after_stable_window() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 1_000L, offlineStableMs = 500L)
        assertEquals(ConnectivityState.Connecting, gate.state.value)

        gate.onValidatedChanged(true)
        runCurrent()
        assertEquals(ConnectivityState.Connecting, gate.state.value)

        advanceTimeBy(999L)
        runCurrent()
        assertEquals(ConnectivityState.Connecting, gate.state.value)

        advanceTimeBy(1L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)
    }

    @Test
    fun given_connecting_when_redundant_true_then_timer_does_not_restart() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 1_000L, offlineStableMs = 500L)
        gate.onValidatedChanged(true)
        runCurrent()
        advanceTimeBy(800L)
        runCurrent()
        // Xiaomi-style storm: repeated true must not reset the 1s window.
        gate.onValidatedChanged(true)
        gate.onValidatedChanged(true)
        runCurrent()
        advanceTimeBy(200L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)
    }

    @Test
    fun given_online_when_brief_loss_then_stays_online_until_offline_stable() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 100L, offlineStableMs = 800L)
        gate.onValidatedChanged(true)
        advanceTimeBy(100L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)

        gate.onValidatedChanged(false)
        runCurrent()
        advanceTimeBy(400L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)

        gate.onValidatedChanged(true)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)
    }

    @Test
    fun given_online_when_validated_lost_stable_then_offline() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 100L, offlineStableMs = 500L)
        gate.onValidatedChanged(true)
        advanceTimeBy(100L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)

        gate.onValidatedChanged(false)
        runCurrent()
        advanceTimeBy(500L)
        runCurrent()
        assertEquals(ConnectivityState.Offline, gate.state.value)
    }

    @Test
    fun given_offline_when_promoteOnlineNow_then_online_immediate() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 2_000L, offlineStableMs = 100L)
        gate.onValidatedChanged(false)
        advanceTimeBy(100L)
        runCurrent()
        assertEquals(ConnectivityState.Offline, gate.state.value)

        gate.promoteOnlineNow()
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)
    }
}
