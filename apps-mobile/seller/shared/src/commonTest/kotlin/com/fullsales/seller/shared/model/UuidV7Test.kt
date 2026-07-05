package com.fullsales.seller.shared.model

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class UuidV7Test {
    @Test
    fun generateUuidV7_hasVersion7AndValidFormat() {
        val id = generateUuidV7(clockMs = 1_700_000_000_000L)
        assertEquals(36, id.length)
        assertEquals('7', id[14])
        assertTrue(id.matches(UUID_PATTERN))
    }

    private companion object {
        val UUID_PATTERN = Regex(
            "^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",
        )
    }
}
