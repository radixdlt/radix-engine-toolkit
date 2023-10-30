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
        create<MavenPublication>("mavenGithub") {
            groupId = "com.radixdlt"
            artifactId = "radix-engine-toolkit-kotlin"
            version = providers.gradleProperty("ret-version").getOrNull()

            from(components["java"])
        }
        create<MavenPublication>("mavenCentral") {
            groupId = "com.radixdlt"
            artifactId = "radix-engine-toolkit-kotlin"
            version = providers.gradleProperty("ret-version").getOrNull()

            from(components["java"])
            pom {
                name = "Radix engine toolkit"
                url = "https://github.com/radixdlt/radix-engine-toolkit"
                licenses {
                    license {
                        name = "Apache License, Version 2.0"
                        url = "http://www.apache.org/licenses/LICENSE-2.0.txt"
                    }
                    license {
                        name = "Radix Eula"
                        url = "https://www.radixdlt.com/terms/genericEULA"
                    }
                }
            }
        }
    }

    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/radixdlt/radix-engine-toolkit")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }

        maven {
            name = "mavenCentral"
            url = uri("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/")
            credentials {
                username = project.findProperty("ossrhUsername") as String?
                password = project.findProperty("ossrhPassword") as String?
            }
        }
    }
}

signing {
    sign(publishing.publications["mavenCentral"])
}
