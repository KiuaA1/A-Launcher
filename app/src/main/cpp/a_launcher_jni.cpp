#include <jni.h>

extern "C" int native_engine_ready() __attribute__((weak));

extern "C"
JNIEXPORT jint JNICALL
Java_com_kiuaa1_alauncher_JniNativeLauncherBridge_nativeLaunch(
        JNIEnv* env,
        jobject /* thiz */,
        jstring instance_id) {
    if (instance_id == nullptr) return 0;

    const char* raw = env->GetStringUTFChars(instance_id, nullptr);
    if (raw == nullptr || raw[0] == '\0') {
        if (raw != nullptr) env->ReleaseStringUTFChars(instance_id, raw);
        return 0;
    }

    const int engineReady = native_engine_ready ? native_engine_ready() : 0;
    env->ReleaseStringUTFChars(instance_id, raw);
    return engineReady ? 1 : 0; // Preparing only when Rust is linked
}
