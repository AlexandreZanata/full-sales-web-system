package com.fullsales.seller.shared.model

enum class LocalRegistrationSyncStatus {
    PendingSync,
    Synced,
    SyncFailed,
}

/**
 * LocalStore row for commerce registration (Phase 16C).
 * OD-16-3 style: mirrored remotes use localId = remoteId.
 */
data class LocalRegistration(
    val localId: String,
    val remoteId: String? = null,
    val cnpj: String,
    val legalName: String,
    val tradeName: String,
    val active: Boolean = false,
    val registrationStatus: String = "PendingReview",
    val rejectionReason: String? = null,
    val registrationMode: String? = null,
    val contactPhone: String? = null,
    val contactEmail: String? = null,
    val deliveryAddressJson: String = "{}",
    val syncStatus: LocalRegistrationSyncStatus,
    val syncFailureReason: String? = null,
    val createdAtEpochMs: Long,
    val updatedAtEpochMs: Long,
    val idempotencyKey: String,
)

fun LocalRegistration.toCommerceRegistration(): CommerceRegistration =
    CommerceRegistration(
        id = remoteId ?: localId,
        cnpj = cnpj,
        legalName = legalName,
        tradeName = tradeName,
        active = active,
        registrationStatus = when (syncStatus) {
            LocalRegistrationSyncStatus.PendingSync -> "PendingSync"
            LocalRegistrationSyncStatus.SyncFailed -> "SyncFailed"
            LocalRegistrationSyncStatus.Synced -> registrationStatus
        },
        rejectionReason = rejectionReason,
        registrationMode = registrationMode,
    )
