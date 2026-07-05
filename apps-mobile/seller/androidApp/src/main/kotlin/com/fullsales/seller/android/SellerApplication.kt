package com.fullsales.seller.android

import android.app.Application

class SellerApplication : Application() {
    lateinit var container: AppContainer
        private set

    override fun onCreate() {
        super.onCreate()
        container = AppContainer(this)
        container.scheduleSync()
    }
}
