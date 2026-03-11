group = "ori"
version = "1.0.0"

plugins {
    id("com.android.library") version "8.7.3"
}

android {
    namespace = "ori"
    compileSdk = 35

    defaultConfig {
        minSdk = 21
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
