#!/usr/bin/env bash
# Builds the Rust engine (crate uncertainty-ffi) as static libraries for the
# macOS and iOS applications, which link it with -force_load
# (macos/Runner/Configs/Engine.xcconfig, ios/Flutter/Engine.xcconfig).
#
#   app/tool/build_engine_apple.sh [macos] [ios]     (default: both)
#
# Output: target/apple/{macos,ios,ios-sim}/libuncertainty_ffi.a
set -euo pipefail
cd "$(dirname "$0")/../.."
platforms=("$@")
[ ${#platforms[@]} -eq 0 ] && platforms=(macos ios)
out=target/apple

build() { # target...
  for t in "$@"; do
    rustup target add "$t" >/dev/null
    cargo build --release -p uncertainty-ffi --target "$t"
  done
}

universal() { # destination targets...
  local dest=$1; shift
  mkdir -p "$out/$dest"
  lipo -create $(for t in "$@"; do echo "target/$t/release/libuncertainty_ffi.a"; done) \
    -output "$out/$dest/libuncertainty_ffi.a"
  lipo -info "$out/$dest/libuncertainty_ffi.a"
}

for p in "${platforms[@]}"; do
  case $p in
    macos)
      # Deployment target of macos/Runner.xcodeproj.
      export MACOSX_DEPLOYMENT_TARGET=12.0
      build aarch64-apple-darwin x86_64-apple-darwin
      universal macos aarch64-apple-darwin x86_64-apple-darwin
      ;;
    ios)
      # Deployment target of ios/Runner.xcodeproj.
      export IPHONEOS_DEPLOYMENT_TARGET=15.0
      build aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
      universal ios aarch64-apple-ios
      universal ios-sim aarch64-apple-ios-sim x86_64-apple-ios
      ;;
    *) echo "unknown platform '$p' (macos, ios)" >&2; exit 2 ;;
  esac
done
