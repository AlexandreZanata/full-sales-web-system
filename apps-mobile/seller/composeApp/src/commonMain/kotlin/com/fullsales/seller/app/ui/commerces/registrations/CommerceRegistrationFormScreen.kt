package com.fullsales.seller.app.ui.commerces.registrations

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import com.fullsales.seller.app.ui.shell.NestedScreenScaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings
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
    LaunchedEffect(state.snackbarCode) {
        state.snackbarCode?.let { code ->
            val message = when (code) {
                "SUBMITTED" -> s.registrations.submitted
                "QUEUED" -> s.registrations.queued
                else -> SellerStrings.registrationError(s, code)
            }
            snackbarHostState.showSnackbar(message)
            viewModel.clearSnackbar()
            if (code == "SUBMITTED" || code == "QUEUED") onSubmitted()
        }
    }
    NestedScreenScaffold(snackbarHost = { SnackbarHost(snackbarHostState) }) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
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
                Text(s.registrations.formTitle, style = MaterialTheme.typography.headlineSmall, modifier = Modifier.screenTitle())
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
            Button(
                onClick = viewModel::submit,
                enabled = state.submitEnabled,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(if (state.submitting) s.common.saving else s.registrations.submit)
            }
            TextButton(onClick = onBack, modifier = Modifier.fillMaxWidth()) {
                Text(s.common.back)
            }
        }
    }
}
