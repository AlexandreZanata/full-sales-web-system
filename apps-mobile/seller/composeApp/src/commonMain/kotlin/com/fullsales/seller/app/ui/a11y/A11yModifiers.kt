package com.fullsales.seller.app.ui.a11y

import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription

fun Modifier.screenTitle(): Modifier = semantics { heading() }

fun Modifier.listItemSummary(description: String): Modifier = semantics {
    contentDescription = description
}

fun Modifier.selectableChipA11y(
    label: String,
    selected: Boolean,
    selectedLabel: String,
): Modifier = semantics {
    contentDescription = label
    if (selected) stateDescription = selectedLabel
}
