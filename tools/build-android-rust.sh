#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NDK="${ANDROID_NDK_HOME:-}"
API="${ANDROID_API_LEVEL:-26}"
MODE="${1:-release}"
if [[ -z "$NDK" || ! -d "$NDK" ]]; then echo "ANDROID_NDK_HOME must point to an installed Android NDK." >&2; exit 1; fi
HOST_TAG=""
case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) HOST_TAG="linux-x86_64" ;;
  Darwin-arm64) HOST_TAG="darwin-arm64" ;;
  Darwin-x86_64) HOST_TAG="darwin-x86_64" ;;
  *) echo "Unsupported host for this build script." >&2; exit 1 ;;
esac
LLVM="$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin"
for tool in aarch64-linux-android$API-clang armv7a-linux-androideabi$API-clang x86_64-linux-android$API-clang i686-linux-android$API-clang; do
  [[ -x "$LLVM/$tool" ]] || { echo "Missing NDK toolchain: $tool" >&2; exit 1; }
done
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$LLVM/aarch64-linux-android$API-clang"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$LLVM/armv7a-linux-androideabi$API-clang"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$LLVM/x86_64-linux-android$API-clang"
export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="$LLVM/i686-linux-android$API-clang"
FLAGS=()
[[ "$MODE" == "release" ]] && FLAGS+=(--release)
for target in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
  rustup target add "$target"
  cargo build --manifest-path "$ROOT/engine/android/Cargo.toml" --target "$target" "${FLAGS[@]}"
done
OUT="$ROOT/app/src/main/jniLibs"
mkdir -p "$OUT/arm64-v8a" "$OUT/armeabi-v7a" "$OUT/x86_64" "$OUT/x86"
cp "$ROOT/engine/target/aarch64-linux-android/$MODE/liba_launcher_android.so" "$OUT/arm64-v8a/"
cp "$ROOT/engine/target/armv7-linux-androideabi/$MODE/liba_launcher_android.so" "$OUT/armeabi-v7a/"
cp "$ROOT/engine/target/x86_64-linux-android/$MODE/liba_launcher_android.so" "$OUT/x86_64/"
cp "$ROOT/engine/target/i686-linux-android/$MODE/liba_launcher_android.so" "$OUT/x86/"
