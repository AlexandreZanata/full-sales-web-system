package com.fullsales.seller.shared.media

/** Turns API-relative media paths into absolute URLs for image loaders. */
fun absoluteMediaUrl(url: String, apiBaseUrl: String): String {
    if (url.startsWith("http://") || url.startsWith("https://")) return url
    if (!url.startsWith("/")) return url
    val origin = apiBaseUrl.removeSuffix("/v1").trimEnd('/')
    return origin + url
}

/** Product thumbnails are served on the public media route (no auth headers). */
fun productThumbnailLoadUrl(url: String, apiBaseUrl: String): String {
    val absolute = absoluteMediaUrl(url, apiBaseUrl)
    val match = Regex("""/v1/media/([^/]+)/content""").find(absolute) ?: return absolute
    val fileId = match.groupValues[1]
    return absolute.replace("/v1/media/$fileId/content", "/v1/public/media/$fileId/content")
}
