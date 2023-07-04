import org.gradle.jvm.tasks.Jar

plugins {
    kotlin("jvm") version "1.8.21"
    `java-library`
    `maven-publish`
}

repositories {
    mavenCentral()
}

dependencies {
   implementation("net.java.dev.jna:jna:5.13.0")
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(8))
    }
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "org.radixdlt"
            artifactId = "radix-engine-toolkit-kotlin"
            version = providers.gradleProperty("ret-version").getOrNull()

            from(components["java"])
        }
    }
}
