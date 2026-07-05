package com.fullsales.seller.shared.auth

import com.fullsales.seller.shared.model.currentEpochMs
import kotlin.test.Test
import kotlin.test.assertIs
import kotlin.test.assertEquals

class JwtRoleGateTest {
    @Test
    fun gateSellerAccessToken_acceptsSellerRole() {
        val token = jwt(role = "Seller", exp = currentEpochMs() / 1000 + 3600)
        val result = gateSellerAccessToken(token)
        assertIs<SellerRoleGateResult.Accepted>(result)
        assertEquals("Seller", result.claims.role)
    }

    @Test
    fun gateSellerAccessToken_rejectsDriverRole() {
        val token = jwt(role = "Driver", exp = currentEpochMs() / 1000 + 3600)
        assertIs<SellerRoleGateResult.NotSeller>(gateSellerAccessToken(token))
    }

    private fun jwt(role: String, exp: Long): String {
        val header = encodeBase64Url("{\"alg\":\"none\"}".encodeToByteArray())
        val payload = encodeBase64Url(
            """{"sub":"user-1","role":"$role","exp":$exp}""".encodeToByteArray(),
        )
        return "$header.$payload.sig"
    }
}
