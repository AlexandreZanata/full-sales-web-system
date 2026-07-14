package com.fullsales.seller.shared.registrations

import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.generateUuidV7
import com.fullsales.seller.shared.repository.RegistrationRepository
import com.fullsales.seller.shared.sales.isTransportFailure
import com.fullsales.seller.shared.sync.OfflineRegistrationWriter

sealed class CreateRegistrationSubmitResult {
    data class Success(val navigationId: String, val isRemote: Boolean) : CreateRegistrationSubmitResult()
    data class Failure(val code: String) : CreateRegistrationSubmitResult()
}

class CreateRegistrationSubmitter(
    private val apiClient: SellerApiClient,
    private val offlineWriter: OfflineRegistrationWriter,
    private val registrations: RegistrationRepository,
    private val newIdempotencyKey: () -> String = { generateUuidV7() },
) {
    suspend fun submit(
        request: SubmitRegistrationRequest,
        online: Boolean,
    ): CreateRegistrationSubmitResult = if (online) {
        submitOnline(request)
    } else {
        submitOffline(request)
    }

    private suspend fun submitOnline(request: SubmitRegistrationRequest): CreateRegistrationSubmitResult {
        val key = newIdempotencyKey()
        return runCatching {
            val remote = apiClient.submitRegistration(request, key)
            registrations.upsertSyncedRemote(remote)
            CreateRegistrationSubmitResult.Success(remote.id, isRemote = true)
        }.getOrElse { error ->
            if (isTransportFailure(error)) submitOffline(request, key) else mapError(error)
        }
    }

    private suspend fun submitOffline(
        request: SubmitRegistrationRequest,
        key: String = newIdempotencyKey(),
    ): CreateRegistrationSubmitResult = runCatching {
        val local = offlineWriter.enqueue(request, key)
        CreateRegistrationSubmitResult.Success(local.localId, isRemote = false)
    }.getOrElse {
        CreateRegistrationSubmitResult.Failure("LOCAL_ERROR")
    }

    private fun mapError(error: Throwable): CreateRegistrationSubmitResult.Failure {
        if (error is ApiException) return CreateRegistrationSubmitResult.Failure(error.detail.code)
        return CreateRegistrationSubmitResult.Failure("NETWORK_ERROR")
    }
}
