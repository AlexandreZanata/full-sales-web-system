package com.fullsales.seller.shared.sync

/** ponytail: Phase 66 adds BGTaskScheduler; invoke [onAppResume] from iOS app lifecycle. */
class IosForegroundSync(private val coordinator: SellerSyncCoordinator) {
    suspend fun onAppResume() {
        coordinator.syncPullAndPush()
    }
}
