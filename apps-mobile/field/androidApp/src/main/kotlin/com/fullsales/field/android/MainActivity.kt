package com.fullsales.field.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import com.fullsales.field.android.ui.FieldNavHost
import com.fullsales.field.android.ui.sales.SalesViewModel
import com.fullsales.field.android.ui.sales.SalesViewModelFactory
import com.fullsales.field.android.ui.theme.FieldTheme

class MainActivity : ComponentActivity() {
    private val viewModel: SalesViewModel by viewModels {
        SalesViewModelFactory((application as FieldApplication).container)
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            FieldTheme {
                Surface(modifier = Modifier) {
                    FieldNavHost(viewModel = viewModel)
                }
            }
        }
    }
}
