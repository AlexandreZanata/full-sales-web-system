package com.fullsales.seller.android.i18n

import androidx.lifecycle.ViewModel
import com.fullsales.seller.shared.i18n.SellerLocale
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

class LocaleViewModel(
    private val store: LocaleStore,
) : ViewModel() {
    private val _locale = MutableStateFlow(store.read())
    val locale: StateFlow<SellerLocale> = _locale.asStateFlow()

    fun setLocale(locale: SellerLocale) {
        if (_locale.value == locale) return
        store.write(locale)
        _locale.value = locale
    }
}
