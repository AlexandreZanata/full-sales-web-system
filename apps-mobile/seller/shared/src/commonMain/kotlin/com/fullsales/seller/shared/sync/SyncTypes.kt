package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.SyncOutboxEntry

enum class SyncHttpOutcome {
    Success,
    InsufficientStock,
    Unauthorized,
    NetworkError,
    ClientError,
}

data class SyncHttpResult(
    val outcome: SyncHttpOutcome,
    val remoteId: String? = null,
    val errorCode: String? = null,
)

interface SyncTransport {
    suspend fun execute(entry: SyncOutboxEntry): SyncHttpResult
}

interface SyncTokenRefresher {
    suspend fun refreshToken(): Boolean
}

data class SyncProcessResult(
    val processedCount: Int,
    val stoppedEarly: Boolean = false,
)
