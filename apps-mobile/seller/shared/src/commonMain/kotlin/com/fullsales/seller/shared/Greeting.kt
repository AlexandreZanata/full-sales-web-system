package com.fullsales.seller.shared

class Greeting {
    private val platform = getPlatform()

    fun greet(): String = "Seller app on ${platform.name}"
}
