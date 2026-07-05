package com.fullsales.seller.shared

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform
