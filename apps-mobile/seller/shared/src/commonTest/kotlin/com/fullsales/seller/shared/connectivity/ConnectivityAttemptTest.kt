package com.fullsales.seller.shared.connectivity

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ConnectivityAttemptTest {
    @Test
    fun connecting_can_attempt_network_but_not_internet_only() {
        assertTrue(ConnectivityState.Connecting.canAttemptNetwork())
        assertFalse(ConnectivityState.Connecting.isDefinitelyOffline())
        assertFalse(ConnectivityState.Connecting.allowsInternetOnlyActions())
    }

    @Test
    fun offline_blocks_attempt_and_is_definitely_offline() {
        assertFalse(ConnectivityState.Offline.canAttemptNetwork())
        assertTrue(ConnectivityState.Offline.isDefinitelyOffline())
    }

    @Test
    fun online_allows_attempt_and_internet_only() {
        assertTrue(ConnectivityState.Online.canAttemptNetwork())
        assertTrue(ConnectivityState.Online.allowsInternetOnlyActions())
        assertFalse(ConnectivityState.Online.isDefinitelyOffline())
    }
}
