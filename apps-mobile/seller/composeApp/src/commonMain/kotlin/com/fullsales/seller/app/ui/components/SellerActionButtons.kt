package com.fullsales.seller.app.ui.components

import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.OutlinedButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.theme.SellerCornerRadius
import com.fullsales.seller.app.ui.theme.SellerError
import com.fullsales.seller.app.ui.theme.SellerOnError

@Composable
fun SellerPrimaryButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    leadingIcon: ImageVector? = null,
    fillMaxWidth: Boolean = true,
    content: @Composable RowScope.() -> Unit,
) {
    Button(
        onClick = onClick,
        enabled = enabled,
        modifier = buttonModifier(modifier, fillMaxWidth),
        shape = RoundedCornerShape(SellerCornerRadius),
        content = { ButtonLeadingIcon(leadingIcon); content() },
    )
}

/** Filled vivid-red cancel / destructive action (same shape as confirm). */
@Composable
fun SellerDangerButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    leadingIcon: ImageVector? = null,
    fillMaxWidth: Boolean = true,
    content: @Composable RowScope.() -> Unit,
) {
    Button(
        onClick = onClick,
        enabled = enabled,
        modifier = buttonModifier(modifier, fillMaxWidth),
        shape = RoundedCornerShape(SellerCornerRadius),
        colors = ButtonDefaults.buttonColors(
            containerColor = SellerError,
            contentColor = SellerOnError,
            disabledContainerColor = SellerError.copy(alpha = 0.38f),
            disabledContentColor = SellerOnError.copy(alpha = 0.70f),
        ),
        content = { ButtonLeadingIcon(leadingIcon); content() },
    )
}

@Composable
fun SellerSecondaryButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    leadingIcon: ImageVector? = null,
    fillMaxWidth: Boolean = true,
    content: @Composable RowScope.() -> Unit,
) {
    OutlinedButton(
        onClick = onClick,
        enabled = enabled,
        modifier = buttonModifier(modifier, fillMaxWidth),
        shape = RoundedCornerShape(SellerCornerRadius),
        content = { ButtonLeadingIcon(leadingIcon); content() },
    )
}

private fun buttonModifier(modifier: Modifier, fillMaxWidth: Boolean): Modifier =
    modifier
        .then(if (fillMaxWidth) Modifier.fillMaxWidth() else Modifier)
        .defaultMinSize(minHeight = 52.dp)

@Composable
private fun RowScope.ButtonLeadingIcon(leadingIcon: ImageVector?) {
    if (leadingIcon != null) {
        Icon(leadingIcon, contentDescription = null, modifier = Modifier.size(20.dp))
        Spacer(Modifier.width(8.dp))
    }
}
