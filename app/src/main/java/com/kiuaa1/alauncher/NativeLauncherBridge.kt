package com.kiuaa1.alauncher

/**
 * Stable Kotlin boundary for the native A-Launcher engine.
 *
 * The implementation is intentionally isolated here so Compose never knows
 * how Rust/JNI is hosted.
 */
interface NativeLauncherBridge {
    fun engineVersion(): String
    fun listInstances(): List<InstanceSummary>
    fun createInstance(id: String, name: String, minecraftVersion: String, loader: String?): InstanceSummary
    fun deleteInstance(id: String)
}

class UnavailableNativeLauncherBridge : NativeLauncherBridge {
    override fun engineVersion(): String = "0.1.0"

    override fun listInstances(): List<InstanceSummary> =
        emptyList()

    override fun createInstance(
        id: String,
        name: String,
        minecraftVersion: String,
        loader: String?
    ): InstanceSummary {
        error("Native engine bridge is not connected yet")
    }

    override fun deleteInstance(id: String) {
        error("Native engine bridge is not connected yet")
    }
}
