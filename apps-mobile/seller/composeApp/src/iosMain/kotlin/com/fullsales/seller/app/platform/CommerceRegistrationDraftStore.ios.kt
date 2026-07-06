package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.registrations.CommerceRegistrationDraft
import com.fullsales.seller.shared.registrations.CommerceRegistrationDraftCodec
import platform.Foundation.NSUserDefaults

actual class CommerceRegistrationDraftStore actual constructor() {
    private val defaults = NSUserDefaults.standardUserDefaults

    actual fun read(): CommerceRegistrationDraft? {
        val raw = defaults.stringForKey(KEY_DRAFT) ?: return null
        return CommerceRegistrationDraftCodec.decode(raw)
    }

    actual fun write(draft: CommerceRegistrationDraft) {
        defaults.setObject(CommerceRegistrationDraftCodec.encode(draft), KEY_DRAFT)
    }

    actual fun clear() {
        defaults.removeObjectForKey(KEY_DRAFT)
    }

    private companion object {
        const val KEY_DRAFT = "seller_commerce_registration_draft"
    }
}
