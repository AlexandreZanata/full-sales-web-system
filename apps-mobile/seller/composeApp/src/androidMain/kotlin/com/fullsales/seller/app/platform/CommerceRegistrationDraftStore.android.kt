package com.fullsales.seller.app.platform

import android.content.Context
import com.fullsales.seller.shared.registrations.CommerceRegistrationDraft
import com.fullsales.seller.shared.registrations.CommerceRegistrationDraftCodec

actual class CommerceRegistrationDraftStore actual constructor() {
    private val prefs = appContext.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    actual fun read(): CommerceRegistrationDraft? {
        val raw = prefs.getString(KEY_DRAFT, null) ?: return null
        return CommerceRegistrationDraftCodec.decode(raw)
    }

    actual fun write(draft: CommerceRegistrationDraft) {
        prefs.edit().putString(KEY_DRAFT, CommerceRegistrationDraftCodec.encode(draft)).apply()
    }

    actual fun clear() {
        prefs.edit().remove(KEY_DRAFT).apply()
    }

    private companion object {
        const val PREFS_NAME = "seller_commerce_registration_draft"
        const val KEY_DRAFT = "draft_json"
    }
}
