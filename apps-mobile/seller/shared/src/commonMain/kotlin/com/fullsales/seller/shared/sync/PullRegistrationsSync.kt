package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.CommerceRegistration
import com.fullsales.seller.shared.model.CursorListRegistrations
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.RegistrationRepository

interface RegistrationsPullClient {
    suspend fun fetchRegistrations(limit: Int, cursor: String?): CursorListRegistrations
}

class PullRegistrationsSync(
    private val registrations: RegistrationRepository,
    private val client: RegistrationsPullClient,
    private val pageSize: Int = 50,
    private val maxPages: Int = 50,
) {
    suspend fun pullRegistrations(nowEpochMs: Long = currentEpochMs()) {
        registrations.upsertFromRemote(fetchAll())
        registrations.setLastRegistrationsSyncEpochMs(nowEpochMs)
    }

    private suspend fun fetchAll(): List<CommerceRegistration> {
        val all = mutableListOf<CommerceRegistration>()
        var cursor: String? = null
        var pages = 0
        while (pages < maxPages) {
            pages++
            val page = client.fetchRegistrations(pageSize, cursor)
            if (page.data.isEmpty()) break
            all += page.data
            if (!page.pagination.hasMore || page.pagination.nextCursor == null) break
            cursor = page.pagination.nextCursor
            if (page.data.size < pageSize) break
        }
        return all
    }
}
