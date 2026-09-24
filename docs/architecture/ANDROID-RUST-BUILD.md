# Android Rust engine build

The Android bridge is implemented by `engine/android`.

Set `ANDROID_NDK_HOME`, then run `./tools/build-android-rust.sh`. The script builds arm64-v8a, armeabi-v7a, x86_64, and x86 and copies `liba_launcher_android.so` into `app/src/main/jniLibs/`.

The Kotlin layer loads `a_launcher_android` and calls the JNI methods exported by the Rust Android crate.
