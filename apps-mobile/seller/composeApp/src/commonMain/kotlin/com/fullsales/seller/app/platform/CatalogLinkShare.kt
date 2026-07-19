package com.fullsales.seller.app.platform

/** Native share sheet + clipboard for catalog link (Phase 19F). */
expect object CatalogLinkShare {
    fun shareText(text: String, title: String)

    fun copyToClipboard(text: String, label: String)
}
