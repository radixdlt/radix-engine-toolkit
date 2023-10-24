import org.gradle.jvm.tasks.Jar

plugins {
    kotlin("jvm") version "1.8.21"
    `java-library`
    `maven-publish`
    `signing`
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
            groupId = "com.radixdlt"
            artifactId = "radix-engine-toolkit-kotlin"
            version = providers.gradleProperty("ret-version").getOrNull()

            from(components["java"])
            pom {
                name.set("Radix Engine Toolkit")
                description.set("Radix Engine Toolkit")
                url.set("https://github.com/radixdlt/radix-engine-toolkit.git")
            }
        }
    }

    repositories {

        maven {
            url = uri("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/")
            credentials {
                username = project.findProperty("ossrhUsername") as String?
                password = project.findProperty("ossrhPassword") as String?
            }
        }

    }
}

signing {
    sign(publishing.publications["maven"])
}
