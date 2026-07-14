package com.fullsales.seller.shared.db.entity

import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "registrations")
data class RegistrationEntity(
    @PrimaryKey val localId: String,
    val remoteId: String?,
    val cnpj: String,
    val legalName: String,
    val tradeName: String,
    val active: Boolean,
    val registrationStatus: String,
    val rejectionReason: String?,
    val registrationMode: String?,
    val contactPhone: String?,
    val contactEmail: String?,
    val deliveryAddressJson: String,
    val syncStatus: String,
    val syncFailureReason: String?,
    val createdAtEpochMs: Long,
    val updatedAtEpochMs: Long,
    val idempotencyKey: String,
)
