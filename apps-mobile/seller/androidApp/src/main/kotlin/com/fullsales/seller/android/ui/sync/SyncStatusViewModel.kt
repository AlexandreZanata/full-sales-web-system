package com.fullsales.seller.android.ui.sync

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.fullsales.seller.android.AppContainer
import com.fullsales.seller.shared.model.LocalSale
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.repository.SaleRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

enum class SyncBadge {
    Idle,
    Offline,
    Syncing,
    SyncFailed,
}

class SyncStatusViewModel(
    private val container: AppContainer,
    private val sales: SaleRepository,
    private val outbox: SyncOutboxRepository,
    context: Context,
) : ViewModel() {
    private val connectivity = context.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
    private val _badge = MutableStateFlow(SyncBadge.Idle)
    val badge: StateFlow<SyncBadge> = _badge.asStateFlow()
    private val _refreshing = MutableStateFlow(false)
    val refreshing: StateFlow<Boolean> = _refreshing.asStateFlow()

    init {
        viewModelScope.launch {
            sales.observeSales().collect { updateBadge(it) }
        }
    }

    fun refreshNow() {
        viewModelScope.launch {
            _refreshing.value = true
            _badge.value = SyncBadge.Syncing
            runCatching { container.syncCoordinator.syncPullAndPush() }
            _refreshing.value = false
        }
    }

    private suspend fun updateBadge(localSales: List<LocalSale>) {
        _badge.value = when {
            _refreshing.value -> SyncBadge.Syncing
            localSales.any { it.status == LocalSaleStatus.SyncFailed } -> SyncBadge.SyncFailed
            !isOnline() -> SyncBadge.Offline
            outbox.listPendingFifo().isNotEmpty() -> SyncBadge.Syncing
            else -> SyncBadge.Idle
        }
    }

    private fun isOnline(): Boolean {
        val network = connectivity.activeNetwork ?: return false
        val caps = connectivity.getNetworkCapabilities(network) ?: return false
        return caps.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }
}
