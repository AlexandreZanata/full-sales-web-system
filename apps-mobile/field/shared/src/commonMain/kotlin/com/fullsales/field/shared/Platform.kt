package com.fullsales.field.shared

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform
