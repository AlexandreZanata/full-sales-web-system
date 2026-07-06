package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.sales.CreateSaleDraft

expect class CreateSaleDraftStore() {
    fun read(): CreateSaleDraft?
    fun write(draft: CreateSaleDraft)
    fun clear()
}
