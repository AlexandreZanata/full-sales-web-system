package com.fullsales.field.android.ui

import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.fullsales.field.android.ui.sales.NewSaleScreen
import com.fullsales.field.android.ui.sales.SalesListScreen
import com.fullsales.field.android.ui.sales.SalesViewModel

private object Routes {
    const val LIST = "sales"
    const val NEW = "sales/new"
}

@Composable
fun FieldNavHost(viewModel: SalesViewModel) {
    val navController = rememberNavController()
    NavHost(navController = navController, startDestination = Routes.LIST) {
        composable(Routes.LIST) {
            SalesListScreen(
                viewModel = viewModel,
                onNewSale = { navController.navigate(Routes.NEW) },
            )
        }
        composable(Routes.NEW) {
            NewSaleScreen(
                viewModel = viewModel,
                onBack = { navController.popBackStack() },
                onCreated = { navController.popBackStack() },
            )
        }
    }
}
