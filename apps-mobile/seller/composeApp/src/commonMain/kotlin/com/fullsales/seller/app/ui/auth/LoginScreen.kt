package com.fullsales.seller.app.ui.auth

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Login
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.platform.isDebugBuild
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.components.SellerPrimaryButton
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.i18n.LoginAccessibilityPanel
import com.fullsales.seller.shared.i18n.SellerStrings

@Composable
fun LoginScreen(
    viewModel: AuthViewModel,
    localeViewModel: LocaleViewModel,
    accessibilityViewModel: AccessibilityViewModel,
    onLoggedIn: () -> Unit,
) {
    val state by viewModel.state.collectAsState()
    val locale by localeViewModel.locale.collectAsState()
    val textSizePreset by accessibilityViewModel.preset.collectAsState()
    val s = LocalSellerStrings.current
    var email by rememberSaveable {
        mutableStateOf(if (isDebugBuild()) "seller@test.com" else "")
    }
    var password by rememberSaveable {
        mutableStateOf(if (isDebugBuild()) "secret123" else "")
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .statusBarsPadding()
            .navigationBarsPadding()
            .imePadding()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = 24.dp, vertical = 16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        LoginAccessibilityPanel(
            locale = locale,
            textSizePreset = textSizePreset,
            localeViewModel = localeViewModel,
            accessibilityViewModel = accessibilityViewModel,
            modifier = Modifier.padding(bottom = 8.dp),
        )
        Text(
            s.auth.signInTitle,
            style = MaterialTheme.typography.headlineSmall,
            fontWeight = FontWeight.Bold,
            modifier = Modifier
                .fillMaxWidth()
                .screenTitle(),
        )
        SellerSurfaceCard(contentPadding = false) {
            Column(
                modifier = Modifier.padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                OutlinedTextField(
                    value = email,
                    onValueChange = { email = it },
                    label = { Text(s.auth.email) },
                    modifier = Modifier.fillMaxWidth(),
                    singleLine = true,
                )
                OutlinedTextField(
                    value = password,
                    onValueChange = { password = it },
                    label = { Text(s.auth.password) },
                    modifier = Modifier.fillMaxWidth(),
                    singleLine = true,
                )
            }
        }
        state.error?.let { code ->
            SellerSurfaceCard(contentPadding = false) {
                Text(
                    text = SellerStrings.authError(s, code, state.errorDetail),
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodyMedium,
                    modifier = Modifier.padding(14.dp),
                )
            }
        }
        SellerPrimaryButton(
            onClick = { viewModel.login(email, password, onLoggedIn) },
            enabled = !state.loading && email.isNotBlank() && password.isNotBlank(),
            leadingIcon = if (state.loading) null else Icons.AutoMirrored.Filled.Login,
        ) {
            if (state.loading) {
                CircularProgressIndicator(
                    modifier = Modifier.size(20.dp).padding(2.dp),
                    color = MaterialTheme.colorScheme.onPrimary,
                )
            } else {
                Text(s.auth.signIn)
            }
        }
    }
}
