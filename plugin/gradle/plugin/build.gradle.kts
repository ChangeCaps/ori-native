group = "ori"
version = "1.0.0"

plugins {
    `java-gradle-plugin`
    `maven-publish`
}

gradlePlugin {
    plugins {
        create("oriPlugin") {
            id = "ori.plugin"
            implementationClass = "ori.OriPlugin"
        }
    }
}

dependencies {
    compileOnly(gradleApi())
    compileOnly("com.android.tools.build:gradle:8.7.3")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.15.2")
}
