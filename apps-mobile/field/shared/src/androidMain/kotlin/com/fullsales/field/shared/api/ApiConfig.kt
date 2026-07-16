package com.fullsales.field.shared.api

import com.fullsales.field.shared.BuildConfig

fun interface AuthTokenProvider {
    fun accessToken(): String?
}

/** Resolved at build time via `FIELD_API_BASE_URL` / `field.api.base.url` (Phase 18E). */
val FIELD_API_BASE_URL: String = BuildConfig.API_BASE_URL
