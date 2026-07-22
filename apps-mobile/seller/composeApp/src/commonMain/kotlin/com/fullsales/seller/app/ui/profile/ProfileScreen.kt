package com.fullsales.seller.app.ui.profile

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Checkbox
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.components.SellerPrimaryButton
import com.fullsales.seller.app.ui.components.SellerSurfaceCard
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold

@Composable
fun ProfileScreen(viewModel: ProfileViewModel) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(Unit) { viewModel.load() }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            val message = when (code) {
                "SAVED" -> s.profile.saved
                "SAVE_FAILED" -> s.profile.saveFailed
                "PHONE_INVALID" -> s.profile.phoneInvalid
                "OFFLINE" -> s.common.noConnection
                else -> s.profile.saveFailed
            }
            snackbarHostState.showSnackbar(message)
            viewModel.clearSnackbar()
        }
    }

    NestedScreenScaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) },
    ) { padding ->
        when {
            state.loading -> CircularProgressIndicator(
                modifier = Modifier
                    .padding(padding)
                    .navigationBarsPadding()
                    .padding(24.dp),
            )
            state.errorCode != null -> Text(
                s.profile.loadFailed,
                color = MaterialTheme.colorScheme.error,
                modifier = Modifier
                    .padding(padding)
                    .navigationBarsPadding()
                    .padding(16.dp),
            )
            else -> Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding)
                    .navigationBarsPadding()
                    .verticalScroll(rememberScrollState())
                    .padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                Text(
                    s.profile.title,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.SemiBold,
                )
                SellerSurfaceCard {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(12.dp),
                    ) {
                        OutlinedTextField(
                            value = state.contactPhone,
                            onValueChange = viewModel::setContactPhone,
                            modifier = Modifier.fillMaxWidth(),
                            label = { Text(s.profile.contactPhone) },
                            supportingText = { Text(s.profile.contactPhoneHint) },
                            singleLine = true,
                        )
                        OutlinedTextField(
                            value = state.operatingRegion,
                            onValueChange = viewModel::setOperatingRegion,
                            modifier = Modifier.fillMaxWidth(),
                            label = { Text(s.profile.operatingRegion) },
                            singleLine = true,
                        )
                        if (state.publicCode.isNotBlank()) {
                            OutlinedTextField(
                                value = state.publicCode,
                                onValueChange = {},
                                modifier = Modifier.fillMaxWidth(),
                                label = { Text(s.profile.publicCode) },
                                supportingText = { Text(s.profile.publicCodeHint) },
                                readOnly = true,
                                singleLine = true,
                            )
                        }
                        Row(
                            verticalAlignment = Alignment.CenterVertically,
                            horizontalArrangement = Arrangement.spacedBy(8.dp),
                        ) {
                            Checkbox(
                                checked = state.shareLinkActive,
                                onCheckedChange = viewModel::setShareLinkActive,
                            )
                            Text(s.profile.shareLinkActive)
                        }
                        SellerPrimaryButton(
                            onClick = viewModel::save,
                            enabled = !state.saving,
                        ) {
                            Text(if (state.saving) s.common.saving else s.profile.save)
                        }
                    }
                }
            }
        }
    }
}
