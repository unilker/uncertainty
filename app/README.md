# Uncertainty Calculations — graphical interface / grafik arayüz

Flutter application (desktop and mobile) on top of the Rust engine. The
interface edits the JSON project file and sends it to the engine through a
small C interface (`ffi/`, functions `uc_evaluate`, `uc_validate`,
`uc_template`, `uc_version`); every calculation is done by the validated
Rust core, so the application, the command-line tool and the reports always
agree.

Features: the 71 validated library examples (search, open, recalculate);
new measurement models and top-down evaluations; editing of inputs and
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
| Windows | built by the *Package* workflow on `windows-latest` (widget tests run there against `uncertainty_ffi.dll`) | Visual Studio with the C++ desktop workload, Rust (MSVC); `flutter build windows --release` (the CMake build runs cargo and bundles `uncertainty_ffi.dll`) |
| Android | APK built and checked (engine for arm64-v8a, armeabi-v7a, x86_64) | Android SDK with NDK 28.2, Rust targets `aarch64-linux-android armv7-linux-androideabi x86_64-linux-android`; `flutter build apk --release` (the Gradle task `buildEngine` cross-compiles the engine with the NDK's clang and packages it as `jniLibs`) |
| macOS | planned | build `libuncertainty_ffi.dylib` and add it to the app bundle's Frameworks |
| iOS | planned | link the static library (`crate-type = staticlib`) into the Runner |

The *Package* workflow (`.github/workflows/package.yml`; release tags `v*`
or *Run workflow*) builds the Windows zip, the Linux bundle and the Android
APK and attaches them to the run as artifacts. The APK is signed with the
debug key until a release key is configured.

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
Linux sürümü derlenmiş ve test edilmiştir; Android APK'sı ve Windows paketi *Package* iş akışıyla üretilir;
macOS ve iOS sıradadır.
