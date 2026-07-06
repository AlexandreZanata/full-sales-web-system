package com.fullsales.seller.app.ui.commerces.registrations

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.fullsales.seller.app.ui.a11y.screenTitle
import com.fullsales.seller.app.ui.i18n.LocalSellerStrings

@Composable
fun RegistrationModeScreen(
    onCnpjLookup: () -> Unit,
    onManual: () -> Unit,
    onMyRegistrations: () -> Unit,
) {
    val s = LocalSellerStrings.current
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text(s.registrations.modeTitle, style = MaterialTheme.typography.headlineSmall, modifier = Modifier.screenTitle())
        Text(s.registrations.modeSubtitle, style = MaterialTheme.typography.bodyMedium)
        Button(onClick = onCnpjLookup, modifier = Modifier.fillMaxWidth()) {
            Column {
                Text(s.registrations.modeCnpjLookup)
                Text(s.registrations.modeCnpjLookupHint, style = MaterialTheme.typography.bodySmall)
            }
        }
        OutlinedButton(onClick = onManual, modifier = Modifier.fillMaxWidth()) {
            Column {
                Text(s.registrations.modeManual)
                Text(s.registrations.modeManualHint, style = MaterialTheme.typography.bodySmall)
            }
        }
        OutlinedButton(onClick = onMyRegistrations, modifier = Modifier.fillMaxWidth()) {
            Text(s.registrations.myTitle)
        }
    }
}
