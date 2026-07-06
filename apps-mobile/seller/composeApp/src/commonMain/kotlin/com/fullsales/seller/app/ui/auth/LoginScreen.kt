package com.fullsales.seller.app.ui.auth

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
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
import androidx.compose.ui.unit.dp

import com.fullsales.seller.app.a11y.AccessibilityViewModel
import com.fullsales.seller.app.platform.isDebugBuild
import com.fullsales.seller.app.i18n.LocaleViewModel
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.i18n.LocaleSwitcher
import com.fullsales.seller.app.ui.i18n.TextSizeSwitcher
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
            .padding(24.dp),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 16.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp, Alignment.End),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            LocaleSwitcher(locale = locale, onLocaleChange = localeViewModel::setLocale)
            TextSizeSwitcher(
                preset = textSizePreset,
                onPresetChange = accessibilityViewModel::setPreset,
            )
        }
        Text(
            s.auth.signInTitle,
            style = MaterialTheme.typography.headlineSmall,
            modifier = Modifier.screenTitle(),
        )
        OutlinedTextField(
            value = email,
            onValueChange = { email = it },
            label = { Text(s.auth.email) },
            modifier = Modifier
                .fillMaxWidth()
                .padding(top = 16.dp),
            singleLine = true,
        )
        OutlinedTextField(
            value = password,
            onValueChange = { password = it },
            label = { Text(s.auth.password) },
            modifier = Modifier
                .fillMaxWidth()
                .padding(top = 8.dp),
            singleLine = true,
        )
        state.error?.let { code ->
            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = 12.dp),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.errorContainer,
                ),
            ) {
                Text(
                    text = SellerStrings.authError(s, code, state.errorDetail),
                    color = MaterialTheme.colorScheme.onErrorContainer,
                    style = MaterialTheme.typography.bodyMedium,
                    modifier = Modifier.padding(12.dp),
                )
            }
        }
        Button(
            onClick = { viewModel.login(email, password, onLoggedIn) },
            enabled = !state.loading && email.isNotBlank() && password.isNotBlank(),
            modifier = Modifier
                .fillMaxWidth()
                .defaultMinSize(minHeight = 48.dp)
                .padding(top = 16.dp),
        ) {
            if (state.loading) {
                CircularProgressIndicator(modifier = Modifier.padding(4.dp))
            } else {
                Text(s.auth.signIn)
            }
        }
    }
}
