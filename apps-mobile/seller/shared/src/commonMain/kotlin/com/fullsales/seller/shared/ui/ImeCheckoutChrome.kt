package com.fullsales.seller.shared.ui

/**
 * Create-sale keyboard contract:
 * GIVEN soft-keyboard inset bottom > 0
 * WHEN rendering sticky checkout chrome (total + confirm)
 * THEN hide that chrome so the focused product-search field can remain visible above the IME.
 */
fun shouldHideStickyCheckoutForIme(imeBottomPx: Int): Boolean = imeBottomPx > 0
