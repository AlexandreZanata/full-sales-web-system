package com.fullsales.seller.app.ui.sales

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.FilledIconButton
import androidx.compose.material3.FilledTonalIconButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.OutlinedTextFieldDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.theme.SellerWarningColor

@Composable
internal fun QuantityStepper(
    value: String,
    onValueChange: (String) -> Unit,
    isError: Boolean,
    modifier: Modifier = Modifier,
    isWarning: Boolean = false,
) {
    val s = LocalSellerStrings.current
    val warningColors = if (isWarning && !isError) {
        OutlinedTextFieldDefaults.colors(
            unfocusedBorderColor = SellerWarningColor,
            focusedBorderColor = SellerWarningColor,
            cursorColor = SellerWarningColor,
        )
    } else {
        OutlinedTextFieldDefaults.colors()
    }
    Row(
        modifier = modifier,
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        FilledTonalIconButton(
            onClick = { onValueChange(adjustQuantity(value, -1)) },
            modifier = Modifier.size(44.dp),
            shape = CircleShape,
        ) {
            Icon(Icons.Default.Remove, contentDescription = s.a11y.decreaseQuantity)
        }
        OutlinedTextField(
            value = value,
            onValueChange = { raw ->
                if (raw.isEmpty() || raw.all { it.isDigit() }) {
                    onValueChange(raw)
                }
            },
            isError = isError,
            colors = warningColors,
            modifier = Modifier
                .widthIn(min = 64.dp, max = 88.dp)
                .defaultMinSize(minHeight = 48.dp),
            singleLine = true,
            shape = MaterialTheme.shapes.medium,
            textStyle = MaterialTheme.typography.titleMedium.copy(textAlign = TextAlign.Center),
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
        )
        FilledIconButton(
            onClick = { onValueChange(adjustQuantity(value, 1)) },
            modifier = Modifier.size(44.dp),
            shape = CircleShape,
            colors = IconButtonDefaults.filledIconButtonColors(
                containerColor = MaterialTheme.colorScheme.primary,
                contentColor = MaterialTheme.colorScheme.onPrimary,
            ),
        ) {
            Icon(Icons.Default.Add, contentDescription = s.a11y.increaseQuantity)
        }
    }
}

private fun adjustQuantity(current: String, delta: Int): String {
    val next = (current.toIntOrNull() ?: 0) + delta
    return if (next <= 0) "1" else next.toString()
}
