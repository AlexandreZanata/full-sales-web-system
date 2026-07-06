package com.fullsales.seller.app.ui.i18n

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SegmentedButtonDefaults
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.fullsales.seller.shared.i18n.SellerLocale

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LocaleSwitcher(
    locale: SellerLocale,
    onLocaleChange: (SellerLocale) -> Unit,
    modifier: Modifier = Modifier,
    fillWidth: Boolean = false,
) {
    val options = SellerLocale.entries
    SingleChoiceSegmentedButtonRow(
        modifier = if (fillWidth) modifier.fillMaxWidth() else modifier,
    ) {
        options.forEachIndexed { index, option ->
            SegmentedButton(
                selected = locale == option,
                onClick = { onLocaleChange(option) },
                shape = SegmentedButtonDefaults.itemShape(index = index, count = options.size),
                modifier = if (fillWidth) Modifier.weight(1f) else Modifier,
                label = { Text(option.shortLabel) },
            )
        }
    }
}
