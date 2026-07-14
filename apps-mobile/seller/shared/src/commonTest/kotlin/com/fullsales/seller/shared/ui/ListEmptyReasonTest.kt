package com.fullsales.seller.shared.ui

import com.fullsales.seller.shared.i18n.SellerLocale
import com.fullsales.seller.shared.i18n.SellerStrings
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * T-16-13 — empty LocalStore + never synced offline → bootstrap reason (not generic error).
 */
class ListEmptyReasonTest {
    @Test
    fun given_emptySalesDbNeverSynced_when_offline_then_bootstrapNotGenericError() {
        val reason = resolveListEmptyReason(
            hasLocalRows = false,
            everSynced = false,
            isOnline = false,
            refreshFailed = false,
        )
        assertEquals(ListEmptyReason.OfflineUnavailable, reason)

        val copy = listEmptyCopy(
            SellerStrings.forLocale(SellerLocale.En),
            reason!!,
            ListEmptyDomain.Sales,
        )
        assertTrue(copy.message.contains("online", ignoreCase = true) ||
            copy.message.contains("connect", ignoreCase = true))
        assertNotEquals("Failed to load.", copy.message)
        assertNotEquals("Load failed", copy.title)
    }

    @Test
    fun given_neverSynced_when_online_then_neverSyncedReason() {
        assertEquals(
            ListEmptyReason.NeverSynced,
            resolveListEmptyReason(
                hasLocalRows = false,
                everSynced = false,
                isOnline = true,
                refreshFailed = false,
            ),
        )
    }

    @Test
    fun given_syncedEmpty_when_noRows_then_syncedEmpty() {
        assertEquals(
            ListEmptyReason.SyncedEmpty,
            resolveListEmptyReason(
                hasLocalRows = false,
                everSynced = true,
                isOnline = true,
                refreshFailed = false,
            ),
        )
    }

    @Test
    fun given_localRows_when_refreshFails_then_keepCacheSnackbarReason() {
        assertEquals(
            ListEmptyReason.RefreshFailedKeepCache,
            resolveListEmptyReason(
                hasLocalRows = true,
                everSynced = true,
                isOnline = true,
                refreshFailed = true,
            ),
        )
    }

    @Test
    fun given_localRows_when_refreshOk_then_noEmptyReason() {
        assertNull(
            resolveListEmptyReason(
                hasLocalRows = true,
                everSynced = true,
                isOnline = true,
                refreshFailed = false,
            ),
        )
    }

    @Test
    fun given_refreshFailedKeepCache_when_mapSnackbar_then_i18nNotRawCode() {
        val en = SellerStrings.forLocale(SellerLocale.En)
        val msg = listSnackbarMessage(en, "REFRESH_FAILED")
        assertNotEquals("REFRESH_FAILED", msg)
        assertTrue(msg.isNotBlank())
    }
}
