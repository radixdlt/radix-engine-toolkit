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
    withJavadocJar()
    withSourcesJar()
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
                name = "Radix Engine Toolkit"
                description = "The Radix Engine Toolkit is a library that exposes a set of functions to help clients written in kotlin to compile and decompile transactions, perform SBOR encoding and decoding, derive virtual account and virtual badge addresses, and other functionalities"
                url = "https://github.com/radixdlt/radix-engine-toolkit"
                developers {
                    developer {
                        id = "0xOmarA"
                        name = "Omar Abdulla"
                        email = "omar.abdulla@rdx.works"
                    }
                }
                scm {
                    connection = "scm:git:git://github.com:radixdlt/radix-engine-toolkit.git"
                    developerConnection = "scm:git:ssh://github.com:radixdlt/radix-engine-toolkit.git"
                    url = "https://github.com/radixdlt/radix-engine-toolkit.git"
                }
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
            url = uri("https://ossrh-staging-api.central.sonatype.com/service/local/staging/deploy/maven2/")
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
