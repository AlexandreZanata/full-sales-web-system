plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.ksp)
}

import java.util.Properties

val localProperties = Properties().apply {
    val file = rootProject.file("local.properties")
    if (file.exists()) {
        file.inputStream().use { load(it) }
    }
}
val fieldApiBaseUrl: String = System.getenv("FIELD_API_BASE_URL")
    ?: localProperties.getProperty("field.api.base.url")
    ?: "http://10.0.2.2:8080/v1"

kotlin {
    androidTarget {
        compilations.all {
            compileTaskProvider.configure {
                compilerOptions {
                    jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17)
                }
            }
        }
    }

    iosArm64()
    iosSimulatorArm64()

    sourceSets {
        commonMain.dependencies {
            implementation(libs.kotlinx.serialization.json)
            implementation(libs.kotlinx.coroutines.core)
            implementation(libs.ktor.client.core)
            implementation(libs.ktor.client.content.negotiation)
            implementation(libs.ktor.serialization.kotlinx.json)
        }
        commonTest.dependencies {
            implementation(libs.kotlin.test)
            implementation(libs.kotlinx.coroutines.core)
            implementation(libs.kotlinx.coroutines.test)
        }
        androidMain.dependencies {
            api(libs.ktor.client.android)
            api(libs.room.runtime)
            api(libs.room.ktx)
            api(libs.kotlinx.coroutines.android)
            api(libs.kotlinx.serialization.json)
            api(libs.ktor.client.core)
        }
    }
}

dependencies {
    add("kspAndroid", libs.room.compiler)
}

android {
    namespace = "com.fullsales.field.shared"
    compileSdk = libs.versions.android.compileSdk.get().toInt()

    buildFeatures {
        buildConfig = true
    }

    defaultConfig {
        minSdk = libs.versions.android.minSdk.get().toInt()
        buildConfigField("String", "API_BASE_URL", "\"$fieldApiBaseUrl\"")
    }

    buildTypes {
        debug {
            buildConfigField("String", "API_BASE_URL", "\"$fieldApiBaseUrl\"")
        }
        release {
            buildConfigField("String", "API_BASE_URL", "\"https://api.fullsales.example/v1\"")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
