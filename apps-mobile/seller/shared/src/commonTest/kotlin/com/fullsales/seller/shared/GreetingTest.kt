package com.fullsales.seller.shared

import kotlin.test.Test
import kotlin.test.assertTrue

class GreetingTest {
    @Test
    fun greetContainsSellerApp() {
        assertTrue(Greeting().greet().contains("Seller app"))
    }
}
