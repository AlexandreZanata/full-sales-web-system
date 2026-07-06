package com.fullsales.seller.app.platform

import com.fullsales.seller.shared.registrations.CommerceRegistrationDraft

expect class CommerceRegistrationDraftStore() {
    fun read(): CommerceRegistrationDraft?
    fun write(draft: CommerceRegistrationDraft)
    fun clear()
}
