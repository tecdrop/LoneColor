plugins {
    alias(libs.plugins.android.application)
}

android {
    namespace = "com.appgramming.lonecolor"
    compileSdk {
        version = release(36)
    }

    defaultConfig {
        applicationId = "com.appgramming.lonecolor"
        minSdk = 21
        targetSdk = 36
        versionCode = 8
        versionName = "2.3.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            // Enables code shrinking, obfuscation, and optimization
            isMinifyEnabled = true

            // Enables resource shrinking (removes unused XML, drawables, etc.)
            isShrinkResources = true

            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
}

dependencies {
}