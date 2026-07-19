package com.fullsales.seller.app.ui.commerces.registrations

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SnackbarDuration
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.components.SellerPrimaryButton
import com.fullsales.seller.app.ui.components.SellerSecondaryButton
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
import com.fullsales.seller.shared.i18n.SellerStrings

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CommerceRegistrationFormScreen(
    viewModel: CommerceRegistrationViewModel,
    onSubmitted: () -> Unit,
    onBack: () -> Unit,
) {
    val s = LocalSellerStrings.current
    val state by viewModel.state.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    var feedbackSuccess by remember { mutableStateOf(false) }
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            val success = code == "SUBMITTED" || code == "QUEUED"
            feedbackSuccess = success
            val message = when (code) {
                "SUBMITTED" -> s.registrations.submitted
                "QUEUED" -> s.registrations.queued
                else -> SellerStrings.registrationError(s, code)
            }
            snackbarHostState.showSnackbar(message, duration = SnackbarDuration.Short)
            viewModel.clearSnackbar()
            if (success) onSubmitted()
        }
    }
    NestedScreenScaffold { padding ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .imePadding()
                .navigationBarsPadding(),
        ) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState())
                    .padding(horizontal = 16.dp)
                    .padding(top = 4.dp, bottom = 16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Text(
                        s.registrations.formTitle,
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.screenTitle(),
                    )
                    if (state.hasPersistedContent) {
                        TextButton(onClick = viewModel::clearForm) { Text(s.registrations.clearForm) }
                    }
                }
                CommerceRegistrationFormFields(
                    draft = state.draft,
                    errors = state.errors,
                    cnpjReadOnly = state.cnpjReadOnly,
                    onDraftChange = viewModel::updateDraft,
                )
                SellerPrimaryButton(
                    onClick = viewModel::submit,
                    enabled = state.submitEnabled,
                    leadingIcon = if (state.submitting) null else Icons.Default.Check,
                ) {
                    Text(if (state.submitting) s.common.saving else s.registrations.submit)
                }
                SellerSecondaryButton(
                    onClick = onBack,
                    leadingIcon = Icons.AutoMirrored.Filled.ArrowBack,
                ) {
                    Text(s.common.back)
                }
            }
            RegistrationFeedbackHost(
                hostState = snackbarHostState,
                success = feedbackSuccess,
                modifier = Modifier
                    .align(Alignment.TopCenter)
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp)
                    .padding(top = 8.dp),
            )
        }
    }
}

@Composable
private fun RegistrationFeedbackHost(
    hostState: SnackbarHostState,
    success: Boolean,
    modifier: Modifier = Modifier,
) {
    val container = if (success) {
        MaterialTheme.colorScheme.primaryContainer
    } else {
        MaterialTheme.colorScheme.errorContainer
    }
    val content = if (success) {
        MaterialTheme.colorScheme.onPrimaryContainer
    } else {
        MaterialTheme.colorScheme.onErrorContainer
    }
    SnackbarHost(hostState = hostState, modifier = modifier) { data ->
        Surface(
            modifier = Modifier.fillMaxWidth(),
            shape = MaterialTheme.shapes.medium,
            color = container,
            contentColor = content,
            shadowElevation = 4.dp,
            tonalElevation = 2.dp,
        ) {
            Row(
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 14.dp),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                if (success) {
                    Icon(Icons.Default.CheckCircle, contentDescription = null, tint = content)
                }
                Text(
                    data.visuals.message,
                    style = MaterialTheme.typography.bodyMedium,
                    fontWeight = FontWeight.Medium,
                    modifier = Modifier.weight(1f),
                )
            }
        }
    }
}
