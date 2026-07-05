package com.fullsales.seller.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import com.fullsales.seller.android.ui.SellerNavHost
import com.fullsales.seller.android.ui.theme.SellerTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val container = (application as SellerApplication).container
        setContent {
            SellerTheme {
                Surface(modifier = Modifier) {
                    SellerNavHost(container = container)
                }
            }
        }
    }

    override fun onResume() {
        super.onResume()
        (application as SellerApplication).container.requestSync()
    }
}
