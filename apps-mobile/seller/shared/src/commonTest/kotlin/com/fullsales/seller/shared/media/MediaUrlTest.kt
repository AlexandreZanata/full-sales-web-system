package com.fullsales.seller.shared.media

import kotlin.test.Test
import kotlin.test.assertEquals

class MediaUrlTest {
    @Test
    fun absoluteMediaUrl_whenRelative_thenPrefixesApiOrigin() {
        val url = absoluteMediaUrl(
            "/v1/public/media/abc/content",
            "http://192.168.0.10:8080/v1",
        )
        assertEquals("http://192.168.0.10:8080/v1/public/media/abc/content", url)
    }

    @Test
    fun productThumbnailLoadUrl_whenAuthenticatedPath_thenUsesPublicRoute() {
        val url = productThumbnailLoadUrl(
            "/v1/media/abc/content",
            "http://192.168.0.10:8080/v1",
        )
        assertEquals("http://192.168.0.10:8080/v1/public/media/abc/content", url)
    }
}
