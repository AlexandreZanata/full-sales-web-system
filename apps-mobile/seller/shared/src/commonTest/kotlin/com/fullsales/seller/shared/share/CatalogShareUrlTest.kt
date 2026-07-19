package com.fullsales.seller.shared.share

import kotlin.test.Test
import kotlin.test.assertEquals

class CatalogShareUrlTest {
    @Test
    fun builds_absolute_url_from_origin_and_path() {
        assertEquals(
            "http://192.168.15.15:5175/s/dev-seller",
            buildCatalogShareUrl("http://192.168.15.15:5175", "/s/dev-seller"),
        )
    }

    @Test
    fun trims_trailing_slash_on_origin() {
        assertEquals(
            "https://portal.example/s/maria",
            buildCatalogShareUrl("https://portal.example/", "s/maria"),
        )
    }
}
