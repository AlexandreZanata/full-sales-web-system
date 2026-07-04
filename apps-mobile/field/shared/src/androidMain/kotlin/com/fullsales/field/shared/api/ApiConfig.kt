package com.fullsales.field.shared.api

fun interface AuthTokenProvider {
    fun accessToken(): String?
}

const val FIELD_API_BASE_URL = "http://10.0.2.2:8080/v1"
