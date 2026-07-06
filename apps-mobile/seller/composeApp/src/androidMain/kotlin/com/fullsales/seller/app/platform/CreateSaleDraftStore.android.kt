package com.fullsales.seller.app.platform

import android.content.Context
import com.fullsales.seller.shared.sales.CreateSaleDraft
import com.fullsales.seller.shared.sales.CreateSaleDraftCodec

actual class CreateSaleDraftStore actual constructor() {
    private val prefs = appContext.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    actual fun read(): CreateSaleDraft? {
        val raw = prefs.getString(KEY_DRAFT, null) ?: return null
        return CreateSaleDraftCodec.decode(raw)
    }

    actual fun write(draft: CreateSaleDraft) {
        prefs.edit().putString(KEY_DRAFT, CreateSaleDraftCodec.encode(draft)).apply()
    }

    actual fun clear() {
        prefs.edit().remove(KEY_DRAFT).apply()
    }

    private companion object {
        const val PREFS_NAME = "seller_create_sale_draft"
        const val KEY_DRAFT = "draft_json"
    }
}
