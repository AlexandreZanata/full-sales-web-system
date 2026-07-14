package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.SaleDisplayStatus

fun remoteSaleStatusToDisplay(status: String): SaleDisplayStatus = when (status) {
    "Confirmed" -> SaleDisplayStatus.Confirmed
    "Cancelled" -> SaleDisplayStatus.Cancelled
    else -> SaleDisplayStatus.Pending
}

fun localSaleStatusToDisplay(status: LocalSaleStatus): SaleDisplayStatus = when (status) {
    LocalSaleStatus.SyncFailed -> SaleDisplayStatus.SyncFailed
    LocalSaleStatus.Cancelled -> SaleDisplayStatus.Cancelled
    LocalSaleStatus.Confirmed -> SaleDisplayStatus.Confirmed
    LocalSaleStatus.Synced -> SaleDisplayStatus.Pending
    LocalSaleStatus.PendingSync, LocalSaleStatus.Draft -> SaleDisplayStatus.PendingSync
}

fun isLocalOnlySale(status: LocalSaleStatus, remoteId: String?): Boolean =
    remoteId == null ||
        status == LocalSaleStatus.PendingSync ||
        status == LocalSaleStatus.SyncFailed ||
        status == LocalSaleStatus.Draft

fun saleDisplaySortRank(status: SaleDisplayStatus): Int =
    if (status == SaleDisplayStatus.PendingSync || status == SaleDisplayStatus.SyncFailed) 0 else 1
