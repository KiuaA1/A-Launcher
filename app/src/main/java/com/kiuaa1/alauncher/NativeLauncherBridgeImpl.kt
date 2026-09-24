package com.kiuaa1.alauncher

class NativeLauncherBridgeImpl(
    private val rootPath: String
) : NativeLauncherBridge {

    init {
        check(nativeInit(rootPath) == 0) { "Failed to initialize A-Launcher native engine" }
    }

    override fun engineVersion(): String = nativeEngineVersion()

    override fun listInstances(): List<InstanceSummary> =
        emptyList() // JSON decoding is added in the next bridge step.

    override fun createInstance(
        id: String,
        name: String,
        minecraftVersion: String,
        loader: String?
    ): InstanceSummary {
        error("Native createInstance bridge is not implemented yet")
    }

    override fun deleteInstance(id: String) {
        error("Native deleteInstance bridge is not implemented yet")
    }

    private external fun nativeInit(rootPath: String): Int
    private external fun nativeEngineVersion(): String

    companion object {
        init {
            System.loadLibrary("a_launcher_android")
        }
    }
}
