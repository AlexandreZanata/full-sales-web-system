plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.android.library)
    alias(libs.plugins.jetbrains.compose)
    alias(libs.plugins.compose.compiler)
}

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
    iosX64()
    iosArm64()
    iosSimulatorArm64()

    listOf(iosX64(), iosArm64(), iosSimulatorArm64()).forEach { target ->
        target.binaries.framework {
            baseName = "ComposeApp"
            isStatic = true
        }
    }

    sourceSets {
        commonMain.dependencies {
            implementation(project(":shared"))
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material3)
            implementation(compose.materialIconsExtended)
            implementation(compose.ui)
            implementation(libs.androidx.lifecycle.viewmodel.compose)
            implementation(libs.androidx.navigation.compose)
            implementation(libs.kotlinx.coroutines.core)
        }
        androidMain.dependencies {
            implementation(libs.kotlinx.coroutines.android)
            implementation("io.coil-kt:coil-compose:2.7.0")
        }
    }
}

android {
    namespace = "com.fullsales.seller.composeapp"
    compileSdk = libs.versions.android.compileSdk.get().toInt()
    defaultConfig {
        minSdk = libs.versions.android.minSdk.get().toInt()
        buildConfigField("boolean", "IS_DEBUG", "false")
    }
    buildFeatures {
        buildConfig = true
    }
    buildTypes {
        debug {
            buildConfigField("boolean", "IS_DEBUG", "true")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
