#include <jni.h>

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

    // Thin JNI boundary. The next native step replaces this stub with
    // the Rust LauncherEngine request.
    env->ReleaseStringUTFChars(instance_id, raw);
    return 1; // Preparing
}
