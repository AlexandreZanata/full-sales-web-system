package com.fullsales.field.android

import android.app.Application

class FieldApplication : Application() {
    lateinit var container: AppContainer
        private set

    override fun onCreate() {
        super.onCreate()
        container = AppContainer(this)
        container.scheduleSync()
    }
}
