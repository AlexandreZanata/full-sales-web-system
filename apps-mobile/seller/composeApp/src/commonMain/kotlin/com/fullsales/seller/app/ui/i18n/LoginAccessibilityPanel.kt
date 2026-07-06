package com.fullsales.seller.app.ui.i18n

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.shared.a11y.TextSizePreset
import com.fullsales.seller.shared.i18n.SellerLocale

@Composable
fun LoginAccessibilityPanel(
    locale: SellerLocale,
    textSizePreset: TextSizePreset,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Text(
                s.a11y.language,
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            LocaleSwitcher(
                locale = locale,
                onLocaleChange = localeViewModel::setLocale,
                fillWidth = true,
            )
        }
        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Text(
                s.a11y.textSizeLabel,
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            TextSizeSwitcher(
                preset = textSizePreset,
                onPresetChange = accessibilityViewModel::setPreset,
            )
        }
    }
}
