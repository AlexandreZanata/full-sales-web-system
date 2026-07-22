package com.fullsales.seller.shared.api

/**
 * OD-21-3 production API default for Android release builds.
 * Staging override via `SELLER_RELEASE_API_BASE_URL` / `seller.release.api.base.url`.
 */
object ApiReleaseDefaults {
    const val PRODUCTION_BASE_URL: String = "https://vendas.comerc.app.br/v1"

    fun isHttpsApiBaseUrl(url: String): Boolean =
        url.startsWith("https://") &&
            !url.contains("10.0.2.2") &&
            !url.contains("127.0.0.1") &&
            !url.contains("localhost")
}
