package com.fullsales.seller.android

import androidx.lifecycle.Lifecycle
import androidx.test.core.app.ActivityScenario
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Test
import org.junit.runner.RunWith

/**
 * OD-21-7 / G6: cold start reaches a resumed Activity (login or session home).
 * Local-only for v1 — run: `./gradlew :androidApp:connectedDebugAndroidTest`
 */
@RunWith(AndroidJUnit4::class)
class LaunchSmokeInstrumentedTest {
    @Test
    fun coldStart_activityResumesWithoutCrash() {
        ActivityScenario.launch(MainActivity::class.java).use { scenario ->
            scenario.moveToState(Lifecycle.State.RESUMED)
            scenario.onActivity { activity ->
                assertNotNull(activity)
                assertEquals(Lifecycle.State.RESUMED, activity.lifecycle.currentState)
            }
        }
    }
}
