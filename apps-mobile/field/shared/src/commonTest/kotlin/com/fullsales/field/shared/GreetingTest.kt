package com.fullsales.field.shared

import kotlin.test.Test
import kotlin.test.assertTrue

class GreetingTest {
    @Test
    fun greetContainsFieldApp() {
        assertTrue(Greeting().greet().contains("Field app"))
    }
}
