package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.SiteSettings
import com.fullsales.seller.shared.model.currentEpochMs
import com.fullsales.seller.shared.repository.SiteSettingsRepository

interface SettingsPullClient {
    suspend fun fetchSettings(): SiteSettings
}

class PullSettingsSync(
    private val repository: SiteSettingsRepository,
    private val client: SettingsPullClient,
) {
    suspend fun pullSettings(nowEpochMs: Long = currentEpochMs()) {
        repository.upsert(client.fetchSettings(), nowEpochMs)
    }
}
