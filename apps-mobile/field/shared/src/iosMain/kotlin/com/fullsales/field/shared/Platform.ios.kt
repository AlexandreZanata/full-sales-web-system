package com.fullsales.field.shared

class IosPlatform : Platform {
    override val name: String = "iOS"
}

actual fun getPlatform(): Platform = IosPlatform()
