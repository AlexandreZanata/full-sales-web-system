package com.fullsales.field.shared

class Greeting {
    private val platform = getPlatform()

    fun greet(): String = "Field app on ${platform.name}"
}
