package com.fullsales.seller.shared.i18n

enum class SellerLocale(val tag: String, val shortLabel: String) {
    En("en", "EN"),
    PtBr("pt-BR", "PT"),
    ;

    companion object {
        val DEFAULT: SellerLocale = PtBr

        fun fromTag(tag: String?): SellerLocale = entries.firstOrNull { it.tag == tag } ?: DEFAULT
    }
}
