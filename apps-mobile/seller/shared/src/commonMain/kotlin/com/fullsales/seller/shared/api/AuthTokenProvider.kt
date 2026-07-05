package com.fullsales.seller.shared.api

fun interface AuthTokenProvider {
    fun accessToken(): String?
}
