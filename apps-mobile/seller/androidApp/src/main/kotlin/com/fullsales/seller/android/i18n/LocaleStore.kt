package com.fullsales.seller.android.i18n

import android.content.Context
import com.fullsales.seller.shared.i18n.SellerLocale

private const val PREFS_NAME = "seller_locale"
private const val KEY_LOCALE = "locale"

class LocaleStore(context: Context) {
    private val prefs = context.applicationContext.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    fun read(): SellerLocale = SellerLocale.fromTag(prefs.getString(KEY_LOCALE, SellerLocale.DEFAULT.tag))

    fun write(locale: SellerLocale) {
        prefs.edit().putString(KEY_LOCALE, locale.tag).apply()
    }
}
