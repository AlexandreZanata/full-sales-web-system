package com.fullsales.seller.app.ui.i18n

import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SegmentedButtonDefaults
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import com.fullsales.seller.shared.a11y.TextSizePreset

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TextSizeSwitcher(
    preset: TextSizePreset,
    onPresetChange: (TextSizePreset) -> Unit,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    val options = TextSizePreset.entries
    SingleChoiceSegmentedButtonRow(
        modifier = modifier.semantics { contentDescription = s.a11y.textSizeLabel },
    ) {
        options.forEachIndexed { index, option ->
            SegmentedButton(
                selected = preset == option,
                onClick = { onPresetChange(option) },
                shape = SegmentedButtonDefaults.itemShape(index = index, count = options.size),
                label = { Text(presetLabel(s, option)) },
            )
        }
    }
}

private fun presetLabel(s: com.fullsales.seller.shared.i18n.SellerMessages, preset: TextSizePreset) =
    when (preset) {
        TextSizePreset.Normal -> s.a11y.textSizeNormal
        TextSizePreset.Large -> s.a11y.textSizeLarge
        TextSizePreset.ExtraLarge -> s.a11y.textSizeExtraLarge
    }
