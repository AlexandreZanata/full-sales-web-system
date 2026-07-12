package com.fullsales.seller.shared.connectivity

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

/**
 * On stable Offline→Online, push outbox once (coalesced). Secondary triggers stay elsewhere.
 */
class OnlineSyncTrigger(
    connectivity: StateFlow<ConnectivityState>,
    private val pushOutbox: suspend () -> Unit,
    scope: CoroutineScope,
) {
    private val mutex = Mutex()

    init {
        scope.launch {
            var wasOnline = connectivity.value.isOnline()
            connectivity
                .map { it.isOnline() }
                .distinctUntilChanged()
                .collect { online ->
                    if (online && !wasOnline) {
                        mutex.withLock {
                            runCatching { pushOutbox() }
                        }
                    }
                    wasOnline = online
                }
        }
    }
}
