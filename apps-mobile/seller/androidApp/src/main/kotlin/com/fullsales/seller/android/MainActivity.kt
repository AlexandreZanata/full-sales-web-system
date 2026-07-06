package com.fullsales.seller.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import com.fullsales.seller.app.SellerRoot

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val container = (application as SellerApplication).container
        setContent {
            SellerRoot(container = container)
        }
    }

    override fun onResume() {
        super.onResume()
        (application as SellerApplication).container.requestSync()
    }
}
