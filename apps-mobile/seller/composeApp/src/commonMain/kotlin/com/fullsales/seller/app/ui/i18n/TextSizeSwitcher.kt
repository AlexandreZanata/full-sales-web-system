package com.fullsales.seller.app.ui.i18n

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.shared.a11y.TextSizePreset
import com.fullsales.seller.shared.i18n.SellerMessages

@OptIn(ExperimentalMaterial3Api::class, ExperimentalLayoutApi::class)
@Composable
fun TextSizeSwitcher(
    preset: TextSizePreset,
    onPresetChange: (TextSizePreset) -> Unit,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    FlowRow(
        modifier = modifier
            .fillMaxWidth()
            .semantics { contentDescription = s.a11y.textSizeLabel },
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        TextSizePreset.entries.forEach { option ->
            val label = textSizePresetLabel(s, option)
            FilterChip(
                selected = preset == option,
                onClick = { onPresetChange(option) },
                label = {
                    Text(
                        label,
                        style = MaterialTheme.typography.labelLarge,
                    )
                },
                modifier = Modifier.selectableChipA11y(
                    label = label,
                    selected = preset == option,
                    selectedLabel = s.a11y.selected,
                ),
            )
        }
    }
}

fun textSizePresetLabel(s: SellerMessages, preset: TextSizePreset): String =
    when (preset) {
        TextSizePreset.Normal -> s.a11y.textSizeNormal
        TextSizePreset.Large -> s.a11y.textSizeLarge
        TextSizePreset.ExtraLarge -> s.a11y.textSizeExtraLarge
    }
