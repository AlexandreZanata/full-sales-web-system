package com.fullsales.seller.shared.auth

import kotlinx.cinterop.ExperimentalForeignApi
import kotlinx.cinterop.alloc
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.ptr
import kotlinx.cinterop.value
import platform.CoreFoundation.CFTypeRefVar
import platform.CoreFoundation.kCFBooleanTrue
import platform.Foundation.NSData
import platform.Foundation.NSString
import platform.Foundation.NSUTF8StringEncoding
import platform.Foundation.create
import platform.Security.SecItemAdd
import platform.Security.SecItemCopyMatching
import platform.Security.SecItemDelete
import platform.Security.kSecAttrAccount
import platform.Security.kSecAttrService
import platform.Security.kSecClass
import platform.Security.kSecClassGenericPassword
import platform.Security.kSecMatchLimit
import platform.Security.kSecMatchLimitOne
import platform.Security.kSecReturnData
import platform.Security.kSecValueData

internal class KeychainTokenStore {
    fun getAccessToken(): String? = read(KEY_ACCESS)

    fun getRefreshToken(): String? = read(KEY_REFRESH)

    fun saveTokens(accessToken: String, refreshToken: String) {
        write(KEY_ACCESS, accessToken)
        write(KEY_REFRESH, refreshToken)
    }

    fun clear() {
        delete(KEY_ACCESS)
        delete(KEY_REFRESH)
    }

    @OptIn(ExperimentalForeignApi::class)
    private fun write(account: String, value: String) {
        delete(account)
        val data = (value as NSString).dataUsingEncoding(NSUTF8StringEncoding) ?: return
        memScoped {
            val query = mapOf<Any?, Any?>(
                kSecClass to kSecClassGenericPassword,
                kSecAttrService to SERVICE,
                kSecAttrAccount to account,
                kSecValueData to data,
            )
            SecItemAdd(query, null)
        }
    }

    @OptIn(ExperimentalForeignApi::class)
    private fun read(account: String): String? = memScoped {
        val result = alloc<CFTypeRefVar>()
        val query = mapOf<Any?, Any?>(
            kSecClass to kSecClassGenericPassword,
            kSecAttrService to SERVICE,
            kSecAttrAccount to account,
            kSecReturnData to kCFBooleanTrue,
            kSecMatchLimit to kSecMatchLimitOne,
        )
        val status = SecItemCopyMatching(query, result.ptr)
        if (status.toInt() != 0) return null
        val data = result.value as? NSData ?: return null
        NSString.create(data, NSUTF8StringEncoding) as String
    }

    @OptIn(ExperimentalForeignApi::class)
    private fun delete(account: String) {
        val query = mapOf<Any?, Any?>(
            kSecClass to kSecClassGenericPassword,
            kSecAttrService to SERVICE,
            kSecAttrAccount to account,
        )
        SecItemDelete(query)
    }

    private companion object {
        const val SERVICE = "com.fullsales.seller.tokens"
        const val KEY_ACCESS = "access_token"
        const val KEY_REFRESH = "refresh_token"
    }
}
