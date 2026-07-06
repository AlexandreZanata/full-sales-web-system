package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.sales.CreateSaleDraft
import com.fullsales.seller.shared.sales.CreateSaleDraftCodec
import platform.Foundation.NSUserDefaults

actual class CreateSaleDraftStore actual constructor() {
    private val defaults = NSUserDefaults.standardUserDefaults

    actual fun read(): CreateSaleDraft? {
        val raw = defaults.stringForKey(KEY_DRAFT) ?: return null
        return CreateSaleDraftCodec.decode(raw)
    }

    actual fun write(draft: CreateSaleDraft) {
        defaults.setObject(CreateSaleDraftCodec.encode(draft), KEY_DRAFT)
    }

    actual fun clear() {
        defaults.removeObjectForKey(KEY_DRAFT)
    }

    private companion object {
        const val KEY_DRAFT = "seller_create_sale_draft"
    }
}
