package com.fullsales.seller.app.ui.shell

import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

/** No extra system-bar padding — [SellerShellScaffold] already handles insets. */
val NestedScreenWindowInsets: WindowInsets = WindowInsets(0, 0, 0, 0)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun NestedScreenScaffold(
    modifier: Modifier = Modifier,
    snackbarHost: @Composable () -> Unit = {},
    floatingActionButton: @Composable () -> Unit = {},
    bottomBar: @Composable () -> Unit = {},
    content: @Composable (PaddingValues) -> Unit,
) {
    Scaffold(
        modifier = modifier,
        contentWindowInsets = NestedScreenWindowInsets,
        snackbarHost = snackbarHost,
        floatingActionButton = floatingActionButton,
        bottomBar = bottomBar,
        content = content,
    )
}
