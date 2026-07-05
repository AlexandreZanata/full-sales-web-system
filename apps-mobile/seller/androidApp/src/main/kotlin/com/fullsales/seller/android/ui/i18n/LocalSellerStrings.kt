package com.fullsales.seller.android.ui.i18n

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.runtime.staticCompositionLocalOf
import com.fullsales.seller.android.i18n.LocaleViewModel
import com.fullsales.seller.shared.i18n.SellerMessages
import com.fullsales.seller.shared.i18n.SellerStrings

val LocalSellerStrings = staticCompositionLocalOf<SellerMessages> {
    error("Seller strings not provided")
}

@Composable
fun SellerStringsProvider(
    localeViewModel: LocaleViewModel,
    content: @Composable () -> Unit,
) {
    val locale by localeViewModel.locale.collectAsState()
    val strings = remember(locale) { SellerStrings.forLocale(locale) }
    CompositionLocalProvider(LocalSellerStrings provides strings, content = content)
}
