package com.fullsales.seller.shared.ui

import com.fullsales.seller.shared.i18n.SellerMessages

enum class ListEmptyDomain {
    Sales,
    Products,
    Commerces,
    Registrations,
}

data class ListEmptyCopy(
    val title: String,
    val message: String,
    val announcement: String,
)

/** Maps [ListEmptyReason] to shared EN/pt-BR copy + TalkBack announcement. */
fun listEmptyCopy(
    messages: SellerMessages,
    reason: ListEmptyReason,
    domain: ListEmptyDomain,
): ListEmptyCopy = when (reason) {
    ListEmptyReason.NeverSynced -> ListEmptyCopy(
        title = messages.common.neverSyncedTitle,
        message = messages.common.neverSyncedMessage,
        announcement = messages.a11y.emptyNeverSynced,
    )
    ListEmptyReason.OfflineUnavailable -> ListEmptyCopy(
        title = messages.common.bootstrapTitle,
        message = messages.common.bootstrapMessage,
        announcement = messages.a11y.emptyOfflineUnavailable,
    )
    ListEmptyReason.SyncedEmpty -> syncedEmptyCopy(messages, domain)
    ListEmptyReason.RefreshFailedKeepCache -> ListEmptyCopy(
        title = messages.common.refreshFailedKeepCache,
        message = messages.common.refreshFailedKeepCache,
        announcement = messages.a11y.refreshKeepCache,
    )
}

fun listSnackbarMessage(messages: SellerMessages, code: String): String = when (code) {
    "OFFLINE" -> messages.common.noConnection
    "REFRESH_FAILED" -> messages.common.refreshFailedKeepCache
    else -> code
}

private fun syncedEmptyCopy(messages: SellerMessages, domain: ListEmptyDomain): ListEmptyCopy =
    when (domain) {
        ListEmptyDomain.Sales -> ListEmptyCopy(
            title = messages.sales.emptyTitle,
            message = messages.sales.emptyMessage,
            announcement = messages.a11y.emptySyncedEmpty,
        )
        ListEmptyDomain.Products -> ListEmptyCopy(
            title = messages.products.emptyTitle,
            message = messages.products.empty,
            announcement = messages.a11y.emptySyncedEmpty,
        )
        ListEmptyDomain.Commerces -> ListEmptyCopy(
            title = messages.commerces.emptyTitle,
            message = messages.commerces.empty,
            announcement = messages.a11y.emptySyncedEmpty,
        )
        ListEmptyDomain.Registrations -> ListEmptyCopy(
            title = messages.registrations.emptyTitle,
            message = messages.registrations.empty,
            announcement = messages.a11y.emptySyncedEmpty,
        )
    }
