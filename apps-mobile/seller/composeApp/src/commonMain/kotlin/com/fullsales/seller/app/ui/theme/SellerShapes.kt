package com.fullsales.seller.app.ui.theme

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Shapes
import androidx.compose.ui.unit.dp

val SellerCornerRadius = 14.dp

val SellerShapes = Shapes(
    extraSmall = RoundedCornerShape(8.dp),
    small = RoundedCornerShape(10.dp),
    medium = RoundedCornerShape(SellerCornerRadius),
    large = RoundedCornerShape(SellerCornerRadius),
    extraLarge = RoundedCornerShape(20.dp),
)
