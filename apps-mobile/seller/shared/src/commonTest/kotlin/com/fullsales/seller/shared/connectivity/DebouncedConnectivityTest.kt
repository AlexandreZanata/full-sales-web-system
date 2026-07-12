package com.fullsales.seller.shared.connectivity

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.advanceTimeBy
import kotlinx.coroutines.test.runCurrent
import kotlinx.coroutines.test.runTest

/**
 * Contract (Phase 14A): Offline immediate; Online only after stable validated window;
 * rapid flaps coalesce to a single Online transition.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class DebouncedConnectivityTest {
    @Test
    fun given_validated_when_flaps_then_single_online_after_stable_window() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 2_000L)
        assertEquals(ConnectivityState.Offline, gate.state.value)

        gate.onValidatedChanged(true)
        runCurrent()
        assertEquals(ConnectivityState.Connecting, gate.state.value)

        gate.onValidatedChanged(false)
        runCurrent()
        assertEquals(ConnectivityState.Offline, gate.state.value)

        gate.onValidatedChanged(true)
        runCurrent()
        assertEquals(ConnectivityState.Connecting, gate.state.value)

        advanceTimeBy(1_999L)
        runCurrent()
        assertEquals(ConnectivityState.Connecting, gate.state.value)

        advanceTimeBy(1L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)
    }

    @Test
    fun given_online_when_validated_lost_then_offline_immediate() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 2_000L)
        gate.onValidatedChanged(true)
        advanceTimeBy(2_000L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)

        gate.onValidatedChanged(false)
        runCurrent()
        assertEquals(ConnectivityState.Offline, gate.state.value)
    }

    @Test
    fun given_connecting_when_flap_restarts_stable_window() = runTest {
        val gate = DebouncedConnectivity(backgroundScope, onlineStableMs = 2_000L)
        gate.onValidatedChanged(true)
        advanceTimeBy(1_500L)
        runCurrent()
        gate.onValidatedChanged(false)
        gate.onValidatedChanged(true)
        runCurrent()
        advanceTimeBy(1_500L)
        runCurrent()
        assertEquals(ConnectivityState.Connecting, gate.state.value)
        advanceTimeBy(500L)
        runCurrent()
        assertEquals(ConnectivityState.Online, gate.state.value)
    }
}
