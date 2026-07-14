package com.fullsales.seller.shared.db.mapper

import com.fullsales.seller.shared.db.entity.RegistrationEntity
import com.fullsales.seller.shared.model.LocalRegistration
import com.fullsales.seller.shared.model.LocalRegistrationSyncStatus

fun RegistrationEntity.toModel(): LocalRegistration = LocalRegistration(
    localId = localId,
    remoteId = remoteId,
    cnpj = cnpj,
    legalName = legalName,
    tradeName = tradeName,
    active = active,
    registrationStatus = registrationStatus,
    rejectionReason = rejectionReason,
    registrationMode = registrationMode,
    contactPhone = contactPhone,
    contactEmail = contactEmail,
    deliveryAddressJson = deliveryAddressJson,
    syncStatus = runCatching { LocalRegistrationSyncStatus.valueOf(syncStatus) }
        .getOrDefault(LocalRegistrationSyncStatus.Synced),
    syncFailureReason = syncFailureReason,
    createdAtEpochMs = createdAtEpochMs,
    updatedAtEpochMs = updatedAtEpochMs,
    idempotencyKey = idempotencyKey,
)

fun LocalRegistration.toEntity(): RegistrationEntity = RegistrationEntity(
    localId = localId,
    remoteId = remoteId,
    cnpj = cnpj,
    legalName = legalName,
    tradeName = tradeName,
    active = active,
    registrationStatus = registrationStatus,
    rejectionReason = rejectionReason,
    registrationMode = registrationMode,
    contactPhone = contactPhone,
    contactEmail = contactEmail,
    deliveryAddressJson = deliveryAddressJson,
    syncStatus = syncStatus.name,
    syncFailureReason = syncFailureReason,
    createdAtEpochMs = createdAtEpochMs,
    updatedAtEpochMs = updatedAtEpochMs,
    idempotencyKey = idempotencyKey,
)
