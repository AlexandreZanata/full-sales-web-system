package com.fullsales.seller.shared.sync

import com.fullsales.seller.shared.model.LocalRegistration
import com.fullsales.seller.shared.model.SubmitRegistrationRequest
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.repository.RegistrationRepository
import com.fullsales.seller.shared.repository.SyncOutboxRepository
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class OfflineRegistrationWriter(
    private val registrations: RegistrationRepository,
    private val outbox: SyncOutboxRepository,
    private val json: Json = Json { ignoreUnknownKeys = true },
) {
    suspend fun enqueue(request: SubmitRegistrationRequest, idempotencyKey: String): LocalRegistration {
        val local = registrations.createPending(request, idempotencyKey)
        outbox.enqueue(
            SyncOutboxEntry(
                id = "${local.localId}:create",
                saleLocalId = local.localId,
                method = "POST",
                path = "/commerces/registrations",
                bodyJson = json.encodeToString(request),
                idempotencyKey = idempotencyKey,
                createdAtEpochMs = local.createdAtEpochMs,
                entityType = SyncEntityType.Registration,
            ),
        )
        return local
    }
}
