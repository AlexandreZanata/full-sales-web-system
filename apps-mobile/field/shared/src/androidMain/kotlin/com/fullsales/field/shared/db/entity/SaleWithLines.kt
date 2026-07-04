package com.fullsales.field.shared.db.entity

import androidx.room.Embedded
import androidx.room.Relation

data class SaleWithLines(
    @Embedded val sale: SaleEntity,
    @Relation(parentColumn = "localId", entityColumn = "saleLocalId")
    val lines: List<SaleLineEntity>,
)
