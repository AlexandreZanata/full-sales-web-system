package com.fullsales.seller.app.ui.shell

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Logout
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.FormatSize
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MenuDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.selectableChipA11y
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.i18n.textSizePresetLabel
import com.fullsales.seller.shared.a11y.TextSizePreset

@Composable
fun ShellOverflowMenu(
    textSizePreset: TextSizePreset,
    onTextSizeChange: (TextSizePreset) -> Unit,
    onLogout: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val s = LocalSellerStrings.current
    var expanded by remember { mutableStateOf(false) }
    Box(modifier = modifier) {
        IconButton(onClick = { expanded = true }) {
            Icon(Icons.Default.MoreVert, contentDescription = s.a11y.menu)
        }
        DropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false },
            modifier = Modifier.widthIn(min = 272.dp),
            shape = MaterialTheme.shapes.extraLarge,
            containerColor = MaterialTheme.colorScheme.surfaceContainerHigh,
            tonalElevation = 8.dp,
            shadowElevation = 4.dp,
        ) {
            Column(modifier = Modifier.padding(vertical = 8.dp)) {
                AccessibilityMenuHeader()
                TextSizePreset.entries.forEach { preset ->
                    TextSizeMenuItem(
                        preset = preset,
                        selected = textSizePreset == preset,
                        onSelect = {
                            onTextSizeChange(preset)
                            expanded = false
                        },
                    )
                }
                HorizontalDivider(
                    modifier = Modifier.padding(horizontal = 12.dp, vertical = 8.dp),
                    color = MaterialTheme.colorScheme.outlineVariant,
                )
                DropdownMenuItem(
                    text = {
                        Text(
                            s.nav.logout,
                            style = MaterialTheme.typography.bodyLarge,
                        )
                    },
                    leadingIcon = {
                        Icon(
                            Icons.AutoMirrored.Filled.Logout,
                            contentDescription = null,
                        )
                    },
                    colors = MenuDefaults.itemColors(
                        textColor = MaterialTheme.colorScheme.error,
                        leadingIconColor = MaterialTheme.colorScheme.error,
                    ),
                    onClick = {
                        expanded = false
                        onLogout()
                    },
                )
            }
        }
    }
}

@Composable
private fun AccessibilityMenuHeader() {
    val s = LocalSellerStrings.current
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Icon(
            Icons.Default.FormatSize,
            contentDescription = null,
            modifier = Modifier.size(20.dp),
            tint = MaterialTheme.colorScheme.primary,
        )
        Text(
            s.a11y.textSizeLabel,
            style = MaterialTheme.typography.titleSmall,
            color = MaterialTheme.colorScheme.onSurface,
        )
    }
}

@Composable
private fun TextSizeMenuItem(
    preset: TextSizePreset,
    selected: Boolean,
    onSelect: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val label = textSizePresetLabel(s, preset)
    DropdownMenuItem(
        text = {
            Text(
                label,
                style = textSizePreviewStyle(preset),
                fontWeight = if (selected) FontWeight.SemiBold else FontWeight.Normal,
                color = if (selected) {
                    MaterialTheme.colorScheme.onPrimaryContainer
                } else {
                    MaterialTheme.colorScheme.onSurface
                },
            )
        },
        leadingIcon = {
            Box(
                modifier = Modifier.size(24.dp),
                contentAlignment = Alignment.Center,
            ) {
                if (selected) {
                    Icon(
                        Icons.Default.Check,
                        contentDescription = s.a11y.selected,
                        tint = MaterialTheme.colorScheme.primary,
                    )
                }
            }
        },
        colors = MenuDefaults.itemColors(
            textColor = MaterialTheme.colorScheme.onSurface,
            leadingIconColor = MaterialTheme.colorScheme.primary,
        ),
        modifier = Modifier
            .selectableChipA11y(label, selected, s.a11y.selected),
        onClick = onSelect,
    )
}

@Composable
private fun textSizePreviewStyle(preset: TextSizePreset): TextStyle =
    when (preset) {
        TextSizePreset.Normal -> MaterialTheme.typography.bodyLarge
        TextSizePreset.Large -> MaterialTheme.typography.titleSmall
        TextSizePreset.ExtraLarge -> MaterialTheme.typography.titleMedium
    }
