package com.fullsales.seller.shared.api

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Contract OD-21-3 / G5: release API default is HTTPS production host (not emulator/LAN).
 */
class ReleaseApiUrlContractTest {
    @Test
    fun productionDefault_isHttpsAndMatchesOd213() {
        assertEquals(
            "https://vendas.comerc.app.br/v1",
            ApiReleaseDefaults.PRODUCTION_BASE_URL,
        )
        assertTrue(ApiReleaseDefaults.isHttpsApiBaseUrl(ApiReleaseDefaults.PRODUCTION_BASE_URL))
    }

    @Test
    fun rejectsCleartextAndEmulatorHosts() {
        assertFalse(ApiReleaseDefaults.isHttpsApiBaseUrl("http://10.0.2.2:8080/v1"))
        assertFalse(ApiReleaseDefaults.isHttpsApiBaseUrl("https://10.0.2.2:8080/v1"))
        assertFalse(ApiReleaseDefaults.isHttpsApiBaseUrl("http://192.168.1.10:8080/v1"))
        assertFalse(ApiReleaseDefaults.isHttpsApiBaseUrl("https://127.0.0.1/v1"))
    }
}
