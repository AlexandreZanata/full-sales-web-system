package com.fullsales.seller.shared.auth

import kotlin.test.Test
import kotlin.test.assertEquals

class Base64UrlTest {
    @Test
    fun roundTrip_preservesJsonPayload() {
        val json = """{"sub":"user-1","role":"Seller","exp":9999999999}"""
        val encoded = encodeBase64Url(json.encodeToByteArray())
        val decoded = decodeBase64Url(encoded).decodeToString()
        assertEquals(json, decoded)
    }
}
