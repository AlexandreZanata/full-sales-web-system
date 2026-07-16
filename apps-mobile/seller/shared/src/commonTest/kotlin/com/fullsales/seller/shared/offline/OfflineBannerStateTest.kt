package com.fullsales.seller.shared.offline

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue
import com.fullsales.seller.shared.connectivity.ConnectivityState

class OfflineBannerStateTest {
    @Test
    fun given_offline_when_resolve_then_banner_visible_network() {
        val state = resolveOfflineBanner(ConnectivityState.Offline, serverUnreachable = false, pendingCount = 2)
        assertTrue(state.visible)
        assertEquals(OfflineBannerReason.Network, state.reason)
        assertTrue(state.showPendingChip)
        assertEquals(2, state.pendingCount)
    }

    @Test
    fun given_online_and_server_unreachable_when_resolve_then_banner_visible_server() {
        val state = resolveOfflineBanner(ConnectivityState.Online, serverUnreachable = true, pendingCount = 0)
        assertTrue(state.visible)
        assertEquals(OfflineBannerReason.Server, state.reason)
        assertFalse(state.showPendingChip)
    }

    @Test
    fun given_online_and_healthy_when_resolve_then_banner_hidden() {
        val state = resolveOfflineBanner(ConnectivityState.Online, serverUnreachable = false, pendingCount = 5)
        assertFalse(state.visible)
        assertEquals(OfflineBannerReason.None, state.reason)
    }

    @Test
    fun given_connecting_when_resolve_then_banner_hidden() {
        val state = resolveOfflineBanner(ConnectivityState.Connecting, serverUnreachable = true, pendingCount = 1)
        assertFalse(state.visible)
    }

    @Test
    fun createSale_mustShowOffline_fields_are_documented() {
        assertTrue(OfflineFieldVisibility.createSaleMustShow.containsAll(
            listOf("commerces", "products", "paymentMethod", "lines", "submit"),
        ))
        assertTrue(OfflineFieldVisibility.internetOnlyDisabledWithWarn.contains("cnpjLookup"))
    }
}
