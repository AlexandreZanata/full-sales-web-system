package com.fullsales.seller.shared.connectivity

/** Validated network reachability for seller UI and sync. */
enum class ConnectivityState {
    Offline,
    Connecting,
    Online,
}

fun ConnectivityState.isOnline(): Boolean = this == ConnectivityState.Online

/** Internet-only actions (CNPJ, registration submit) stay disabled while Connecting. */
fun ConnectivityState.allowsInternetOnlyActions(): Boolean = this == ConnectivityState.Online
