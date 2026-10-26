# Uncertainty Calculations — graphical interface / grafik arayüz

Flutter application (desktop and mobile) on top of the Rust engine. The
interface edits the JSON project file and sends it to the engine through a
small C interface (`ffi/`, functions `uc_evaluate`, `uc_validate`,
`uc_template`, `uc_version`); every calculation is done by the validated
Rust core, so the application, the command-line tool and the reports always
agree.

Features: the 100 validated library examples (search, open, recalculate);
new measurement models, top-down evaluations, proficiency test rounds
(ISO 13528) and calibrations with uncertainties in both variables (ISO 6143); editing of inputs and
distributions, direct entry of raw validation data (control-chart series,
duplicate pairs, points of a precision model), bias, recovery and further
components; GUM, Monte Carlo and reporting settings; the complete JSON file
with validation; results with the reported statement, budget, variance
shares, Monte Carlo histogram with the GUM density, validation of the GUM
result and comparison with reference values; export of HTML and Markdown
reports; English and Turkish.

## Build

Requirements: Flutter (stable, ≥ 3.47), Rust (≥ 1.75).

| Platform | Status | Steps |
|---|---|---|
| Linux | built and tested | needs clang, cmake, ninja, GTK 3 development files; `flutter build linux --release` (the CMake build runs `cargo build --release -p uncertainty-ffi` and bundles `libuncertainty_ffi.so`) |
| Windows | built and tested by the *Package* workflow on `windows-latest` (all widget tests pass against `uncertainty_ffi.dll`; zip with the executable, the engine DLL and the library) | Visual Studio with the C++ desktop workload, Rust (MSVC); `flutter build windows --release` (the CMake build runs cargo and bundles `uncertainty_ffi.dll`) |
| Android | APK built and checked (engine for arm64-v8a, armeabi-v7a, x86_64) | Android SDK with NDK 28.2, Rust targets `aarch64-linux-android armv7-linux-androideabi x86_64-linux-android`; `flutter build apk --release` (the Gradle task `buildEngine` cross-compiles the engine with the NDK's clang and packages it as `jniLibs`) |
| macOS | built by the *Package* workflow on `macos-latest` (widget tests against `libuncertainty_ffi.dylib`; universal arm64/x86_64 app with the engine linked statically) | Xcode, Rust targets `aarch64-apple-darwin x86_64-apple-darwin`; `tool/build_engine_apple.sh macos`, then `flutter build macos --release` (`macos/Runner/Configs/Engine.xcconfig` links `target/apple/macos/libuncertainty_ffi.a` with `-force_load`) |
| iOS | unsigned device build by the *Package* workflow | Xcode, Rust targets `aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios`; `tool/build_engine_apple.sh ios`, then `flutter build ios` (`ios/Flutter/Engine.xcconfig` links the static library for the device or the simulator); signing needs an Apple developer account |

The *Package* workflow (`.github/workflows/package.yml`; release tags `v*`
or *Run workflow*) builds the Windows zip, the Linux bundle, the Android
APK, the macOS application and an unsigned iOS build and attaches them to the
run as artifacts. The macOS application is not notarised (open it with
right-click → Open the first time). The APK is signed with the
debug key until a release key is configured. The Windows build needs the
Microsoft Visual C++ runtime, which is present on most systems (otherwise
install the Visual C++ Redistributable).

```sh
cd app
flutter pub get
flutter test          # needs the engine in ../target/release (cargo build --release -p uncertainty-ffi)
flutter run -d linux
```

Command line: `uncertainty_app [project.json] [--run] [--lang en|tr]` opens a
project (and calculates it).

The example library is bundled as `assets/library.json`; after changing
`../library`, run `dart run tool/bundle_library.dart` (the Rust test suite
fails while the bundle is out of date).

---

Rust motoru üzerinde Flutter uygulaması (masaüstü ve mobil). Arayüz JSON proje
dosyasını düzenler ve küçük bir C arayüzüyle motora gönderir; tüm hesaplar
doğrulanmış Rust çekirdeğinde yapılır. Derleme adımları yukarıdaki tablodadır;
Linux sürümü derlenmiş ve test edilmiştir; Android APK'sı, Windows paketi, macOS uygulaması ve imzasız iOS
derlemesi *Package* iş akışıyla üretilir. iOS'un imzalanması ve dağıtımı Apple geliştirici hesabı gerektirir.
