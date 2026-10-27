import java.io.File
import org.gradle.internal.os.OperatingSystem

plugins {
    id("com.android.application")
    // The Flutter Gradle Plugin must be applied after the Android and Kotlin Gradle plugins.
    id("dev.flutter.flutter-gradle-plugin")
}

// The computation engine (Rust crate uncertainty-ffi), cross-compiled with
// cargo and the NDK's clang for every Android ABI and packaged as jniLibs.
val engineJniLibs: File = layout.buildDirectory.dir("engineJniLibs").get().asFile

android {
    namespace = "org.tubitak.ume.uncertainty_app"
    compileSdk = flutter.compileSdkVersion
    ndkVersion = flutter.ndkVersion

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    defaultConfig {
        applicationId = "org.tubitak.ume.uncertainty_app"
        minSdk = flutter.minSdkVersion
        targetSdk = flutter.targetSdkVersion
        // Uses the version code from pubspec.yaml. When using split APKs, 1000 * ABI_VERSION
        // is added automatically by Flutter. (https://developer.android.com/studio/build/configure-apk-splits#configure-APK-versions)
        // You can force using the value of versionCode by specifying the `-P force-version-code-ignoring-abi=true`
        // flag during build.
        versionCode = flutter.versionCode
        versionName = flutter.versionName
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDir(engineJniLibs)
        }
    }

    buildTypes {
        release {
            // TODO: Add your own signing config for the release build.
            // Signing with the debug keys for now, so `flutter run --release` works.
            signingConfig = signingConfigs.getByName("debug")
        }
    }
}

kotlin {
    compilerOptions {
        jvmTarget = org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17
    }
}

flutter {
    source = "../.."
}

// ABI to (Rust target, clang target prefix).
val engineAbis = mapOf(
    "arm64-v8a" to Pair("aarch64-linux-android", "aarch64-linux-android"),
    "armeabi-v7a" to Pair("armv7-linux-androideabi", "armv7a-linux-androideabi"),
    "x86_64" to Pair("x86_64-linux-android", "x86_64-linux-android"),
)

val buildEngine by tasks.registering {
    description = "Builds libuncertainty_ffi.so for all Android ABIs with cargo."
    val repo = rootProject.projectDir.resolve("../..").canonicalFile
    val api = flutter.minSdkVersion
    inputs.dir(repo.resolve("core/src"))
    inputs.dir(repo.resolve("project/src"))
    inputs.dir(repo.resolve("ffi/src"))
    inputs.file(repo.resolve("Cargo.lock"))
    outputs.dir(engineJniLibs)
    doLast {
        val host = when {
            OperatingSystem.current().isMacOsX -> "darwin-x86_64"
            OperatingSystem.current().isWindows -> "windows-x86_64"
            else -> "linux-x86_64"
        }
        val clangDir = android.ndkDirectory.resolve("toolchains/llvm/prebuilt/$host/bin")
        val suffix = if (OperatingSystem.current().isWindows) ".cmd" else ""
        val cargo = System.getenv("CARGO") ?: File(System.getProperty("user.home"), ".cargo/bin/cargo")
            .takeIf { it.exists() }?.path ?: "cargo"
        for ((abi, triple) in engineAbis) {
            val (rustTarget, clangTarget) = triple
            val linker = clangDir.resolve("$clangTarget$api-clang$suffix")
            val envName = "CARGO_TARGET_" + rustTarget.uppercase().replace('-', '_') + "_LINKER"
            providers.exec {
                workingDir = repo
                environment(envName, linker.path)
                environment("CC_" + rustTarget.replace('-', '_'), linker.path)
                environment("AR_" + rustTarget.replace('-', '_'), clangDir.resolve("llvm-ar").path)
                commandLine(cargo, "build", "--release", "-p", "uncertainty-ffi", "--target", rustTarget)
            }.result.get().assertNormalExitValue()
            val out = engineJniLibs.resolve(abi)
            out.mkdirs()
            repo.resolve("target/$rustTarget/release/libuncertainty_ffi.so").copyTo(out.resolve("libuncertainty_ffi.so"), overwrite = true)
        }
    }
}

tasks.named("preBuild") { dependsOn(buildEngine) }
