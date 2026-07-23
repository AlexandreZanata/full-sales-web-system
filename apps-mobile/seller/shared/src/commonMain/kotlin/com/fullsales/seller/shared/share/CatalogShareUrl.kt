package com.fullsales.seller.shared.share

import com.fullsales.seller.shared.api.catalogBaseUrl
import com.fullsales.seller.shared.model.SellerShare

/** Builds absolute portal catalog URL from origin + API sharePath (`/s/code`). */
fun buildCatalogShareUrl(portalOrigin: String, sharePath: String): String {
    val origin = portalOrigin.trimEnd('/')
    val path = if (sharePath.startsWith("/")) sharePath else "/$sharePath"
    return "$origin$path"
}

/**
 * Prefer seller personal share URL when active; otherwise catalog portal from app env
 * (`SELLER_CATALOG_BASE_URL` / BuildConfig).
 */
fun resolveCatalogShareUrl(share: SellerShare?, portalOrigin: String = catalogBaseUrl): String? {
    val origin = portalOrigin.trim().trimEnd('/')
    if (share != null && share.shareLinkActive) {
        val fromApi = share.shareUrl.trim().takeIf { it.isNotBlank() }
        if (fromApi != null) {
            return fromApi
        }
        val path = share.sharePath.trim()
        if (path.isNotBlank() && origin.isNotBlank()) {
            return buildCatalogShareUrl(origin, path)
        }
    }
    return origin.takeIf { it.isNotBlank() }
}
