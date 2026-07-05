package com.fullsales.seller.shared.sales

import com.fullsales.seller.shared.api.ApiException
import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.generateUuidV7
import com.fullsales.seller.shared.sync.OfflineSaleWriter

sealed class CreateSaleSubmitResult {
    data class Success(val navigationId: String, val isRemote: Boolean) : CreateSaleSubmitResult()
    data class Failure(val code: String, val message: String) : CreateSaleSubmitResult()
}

class CreateSaleSubmitter(
    private val apiClient: SellerApiClient,
    private val offlineWriter: OfflineSaleWriter,
    private val newIdempotencyKey: () -> String = { generateUuidV7() },
) {
    suspend fun submit(
        request: CreateSaleRequest,
        totalAmountMinor: Double,
        online: Boolean,
    ): CreateSaleSubmitResult = if (online) {
        submitOnline(request)
    } else {
        submitOffline(request, totalAmountMinor)
    }

    private suspend fun submitOnline(request: CreateSaleRequest): CreateSaleSubmitResult =
        runCatching {
            val sale = apiClient.createSale(request, newIdempotencyKey())
            CreateSaleSubmitResult.Success(sale.id, isRemote = true)
        }.getOrElse { mapSubmitError(it) }

    private suspend fun submitOffline(
        request: CreateSaleRequest,
        totalAmountMinor: Double,
    ): CreateSaleSubmitResult = runCatching {
        val sale = offlineWriter.createSale(request, totalAmountMinor)
        CreateSaleSubmitResult.Success(sale.localId, isRemote = false)
    }.getOrElse {
        CreateSaleSubmitResult.Failure("LOCAL_ERROR", "LOCAL_ERROR")
    }
}

private fun mapSubmitError(error: Throwable): CreateSaleSubmitResult.Failure {
    if (error is ApiException) {
        return CreateSaleSubmitResult.Failure(
            code = error.detail.code,
            message = error.detail.code,
        )
    }
    return CreateSaleSubmitResult.Failure("NETWORK_ERROR", "NETWORK_ERROR")
}
