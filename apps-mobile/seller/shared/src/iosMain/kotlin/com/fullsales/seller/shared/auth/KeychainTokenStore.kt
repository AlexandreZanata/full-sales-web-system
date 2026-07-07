package com.fullsales.seller.shared.auth

import kotlinx.cinterop.ExperimentalForeignApi
import kotlinx.cinterop.MemScope
import kotlinx.cinterop.alloc
import kotlinx.cinterop.allocArrayOf
import kotlinx.cinterop.convert
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.ptr
import kotlinx.cinterop.reinterpret
import kotlinx.cinterop.value
import platform.CoreFoundation.CFDictionaryCreate
import platform.CoreFoundation.CFDictionaryRef
import platform.CoreFoundation.CFStringRef
import platform.CoreFoundation.CFTypeRef
import platform.CoreFoundation.CFTypeRefVar
import platform.CoreFoundation.kCFAllocatorDefault
import platform.CoreFoundation.kCFBooleanTrue
import platform.Foundation.CFBridgingRelease
import platform.Foundation.CFBridgingRetain
import platform.Foundation.NSData
import platform.Foundation.NSString
import platform.Foundation.NSUTF8StringEncoding
import platform.Foundation.create
import platform.Foundation.dataUsingEncoding
import platform.Security.SecItemAdd
import platform.Security.SecItemCopyMatching
import platform.Security.SecItemDelete
import platform.Security.errSecItemNotFound
import platform.Security.kSecAttrAccount
import platform.Security.kSecAttrService
import platform.Security.kSecClass
import platform.Security.kSecClassGenericPassword
import platform.Security.kSecMatchLimit
import platform.Security.kSecMatchLimitOne
import platform.Security.kSecReturnData
import platform.Security.kSecValueData
import platform.darwin.OSStatus

@OptIn(ExperimentalForeignApi::class)
internal class KeychainTokenStore {
  // ponytail: service CFString retained for process lifetime (multiplatform-settings pattern).
  private val defaults =
      mapOf<CFStringRef?, CFTypeRef?>(
          kSecClass to kSecClassGenericPassword,
          kSecAttrService to CFBridgingRetain(SERVICE),
      )

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

  private fun write(account: String, value: String) {
    delete(account)
    val data = value.toNSString().dataUsingEncoding(NSUTF8StringEncoding) ?: return
    cfRetain(account, data) { (cfAccount, cfData) ->
      keychainOp(
          kSecAttrAccount to cfAccount,
          kSecValueData to cfData,
      ) { SecItemAdd(it, null) }
    }
  }

  private fun read(account: String): String? =
      cfRetain(account) { (cfAccount) ->
        memScoped {
          val result = alloc<CFTypeRefVar>()
          val status =
              keychainOp(
                  kSecAttrAccount to cfAccount,
                  kSecReturnData to kCFBooleanTrue,
                  kSecMatchLimit to kSecMatchLimitOne,
              ) { SecItemCopyMatching(it, result.ptr) }
          if (status == errSecItemNotFound || status.toInt() != 0) {
            return@cfRetain null
          }
          val data = CFBridgingRelease(result.value) as? NSData ?: return@cfRetain null
          NSString.create(data, NSUTF8StringEncoding) as String?
        }
      }

  private fun delete(account: String) {
    cfRetain(account) { (cfAccount) ->
      keychainOp(kSecAttrAccount to cfAccount) { SecItemDelete(it) }
    }
  }

  private inline fun MemScope.keychainOp(
      vararg input: Pair<CFStringRef?, CFTypeRef?>,
      crossinline operation: (CFDictionaryRef?) -> OSStatus,
  ): OSStatus = operation(cfDictionaryOf(defaults + mapOf(*input)))

  private companion object {
    const val SERVICE = "com.fullsales.seller.tokens"
    const val KEY_ACCESS = "access_token"
    const val KEY_REFRESH = "refresh_token"
  }
}

@Suppress("CAST_NEVER_SUCCEEDS")
private fun String.toNSString(): NSString = this as NSString

@OptIn(ExperimentalForeignApi::class)
private inline fun <T> cfRetain(vararg values: Any?, block: MemScope.(Array<CFTypeRef?>) -> T): T =
    memScoped {
      val retained = Array(values.size) { index -> CFBridgingRetain(values[index]) }
      try {
        block(retained)
      } finally {
        retained.forEach { reference -> CFBridgingRelease(reference) }
      }
    }

@OptIn(ExperimentalForeignApi::class)
private fun MemScope.cfDictionaryOf(map: Map<CFStringRef?, CFTypeRef?>): CFDictionaryRef? {
  val keys = allocArrayOf(*map.keys.toTypedArray())
  val values = allocArrayOf(*map.values.toTypedArray())
  return CFDictionaryCreate(
      kCFAllocatorDefault,
      keys.reinterpret(),
      values.reinterpret(),
      map.size.convert(),
      null,
      null,
  )
}
