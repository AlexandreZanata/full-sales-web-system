package com.fullsales.seller.shared.auth

import com.fullsales.seller.shared.model.currentEpochMs
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.Json

data class AccessTokenClaims(
    val sub: String,
    val role: String,
    val exp: Long,
)

sealed class SellerRoleGateResult {
    data class Accepted(val claims: AccessTokenClaims) : SellerRoleGateResult()
    data object NotSeller : SellerRoleGateResult()
    data object InvalidToken : SellerRoleGateResult()
}

private val jwtJson = Json { ignoreUnknownKeys = true }

fun gateSellerAccessToken(token: String, nowEpochMs: Long = currentEpochMs()): SellerRoleGateResult {
    val parts = token.split('.')
    if (parts.size != 3) return SellerRoleGateResult.InvalidToken
    val payloadJson = runCatching {
        decodeBase64Url(parts[1]).decodeToString()
    }.getOrNull() ?: return SellerRoleGateResult.InvalidToken
    val payload = runCatching {
        jwtJson.decodeFromString<JwtPayload>(payloadJson)
    }.getOrNull() ?: return SellerRoleGateResult.InvalidToken
    if (payload.sub.isNullOrBlank() || payload.role.isNullOrBlank() || payload.exp == null) {
        return SellerRoleGateResult.InvalidToken
    }
    if (payload.exp * 1000 <= nowEpochMs) return SellerRoleGateResult.InvalidToken
    if (payload.role != SELLER_ROLE) return SellerRoleGateResult.NotSeller
    return SellerRoleGateResult.Accepted(
        AccessTokenClaims(payload.sub, payload.role, payload.exp),
    )
}

@Serializable
private data class JwtPayload(
    val sub: String? = null,
    val role: String? = null,
    val exp: Long? = null,
    @SerialName("tenant_id") val tenantId: String? = null,
)

const val SELLER_ROLE = "Seller"
