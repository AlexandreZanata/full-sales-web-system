package com.fullsales.seller.shared.share

/** Builds absolute portal catalog URL from origin + API sharePath (`/s/code`). */
fun buildCatalogShareUrl(portalOrigin: String, sharePath: String): String {
    val origin = portalOrigin.trimEnd('/')
    val path = if (sharePath.startsWith("/")) sharePath else "/$sharePath"
    return "$origin$path"
}
