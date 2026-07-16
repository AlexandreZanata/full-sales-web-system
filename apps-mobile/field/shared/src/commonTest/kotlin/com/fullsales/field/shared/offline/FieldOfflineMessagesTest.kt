package com.fullsales.field.shared.offline

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class FieldOfflineMessagesTest {
    @Test
    fun salesEmpty_online_vs_offline() {
        assertEquals("No sales yet", FieldOfflineMessages.salesEmpty(online = true))
        assertEquals("Offline — sync when online", FieldOfflineMessages.salesEmpty(online = false))
    }

    @Test
    fun catalogEmptyOffline_is_explicit() {
        assertTrue(FieldOfflineMessages.catalogEmptyOffline().contains("download catalog"))
    }
}
