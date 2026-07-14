package com.fullsales.seller.shared.ui

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ImeCheckoutChromeTest {
    /** GIVEN IME closed WHEN checkout chrome THEN keep sticky bar. */
    @Test
    fun given_imeClosed_when_checkoutVisible_then_keepStickyBar() {
        assertFalse(shouldHideStickyCheckoutForIme(0))
        assertFalse(shouldHideStickyCheckoutForIme(-1))
    }

    /** GIVEN IME open WHEN searching product THEN hide sticky checkout. */
    @Test
    fun given_imeOpen_when_searchingProduct_then_hideStickyCheckout() {
        assertTrue(shouldHideStickyCheckoutForIme(1))
        assertTrue(shouldHideStickyCheckoutForIme(420))
    }
}

class SoftInputAdjustTest {
    /** GIVEN host uses ADJUST_RESIZE WHEN validating IME forms THEN pass. */
    @Test
    fun given_adjustResize_when_hostConfigured_then_imeFormsPass() {
        val softInputMode = SoftInputAdjust.RESIZE or 0x04 // state visible + adjust resize
        assertEquals(SoftInputAdjust.RESIZE, softInputAdjustMode(softInputMode))
        assertTrue(requiresAdjustResizeForImeForms(softInputMode))
    }

    /** GIVEN host uses ADJUST_PAN/NOTHING WHEN validating IME forms THEN fail. */
    @Test
    fun given_adjustPan_when_hostMisconfigured_then_imeFormsFail() {
        assertFalse(requiresAdjustResizeForImeForms(SoftInputAdjust.PAN))
        assertFalse(requiresAdjustResizeForImeForms(SoftInputAdjust.NOTHING))
        assertFalse(requiresAdjustResizeForImeForms(SoftInputAdjust.UNSPECIFIED))
    }
}
