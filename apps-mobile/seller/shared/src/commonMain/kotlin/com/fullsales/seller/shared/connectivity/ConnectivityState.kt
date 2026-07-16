package com.fullsales.seller.shared.connectivity

/** Validated network reachability for seller UI and sync. */
enum class ConnectivityState {
    Offline,
    Connecting,
    Online,
}

fun ConnectivityState.isOnline(): Boolean = this == ConnectivityState.Online

/** True only for hard Offline — not the Connecting debounce window. */
fun ConnectivityState.isDefinitelyOffline(): Boolean = this == ConnectivityState.Offline

/**
 * Pull/sync may run while Online or Connecting (link already up; gate still stabilizing).
 * Internet-only CTAs still use [allowsInternetOnlyActions].
 */
fun ConnectivityState.canAttemptNetwork(): Boolean =
    this == ConnectivityState.Online || this == ConnectivityState.Connecting

/** Internet-only actions (CNPJ, registration submit) stay disabled while Connecting. */
fun ConnectivityState.allowsInternetOnlyActions(): Boolean = this == ConnectivityState.Online
