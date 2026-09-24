package com.kiuaa1.alauncher

data class InstanceSummary(
    val id: String,
    val name: String,
    val minecraftVersion: String,
    val loader: String?
)

enum class LaunchStatus { Idle, Preparing, Launching, Running, Failed }

interface LauncherApi {
    fun listInstances(): List<InstanceSummary>
    fun createInstance(
        id: String,
        name: String,
        minecraftVersion: String,
        loader: String?
    ): InstanceSummary
    fun deleteInstance(id: String)
    fun launch(instanceId: String): LaunchStatus
}

class LocalLauncherApi(private val native: NativeLauncherBridge = JniNativeLauncherBridge()) : LauncherApi {
    private val instances = linkedMapOf<String, InstanceSummary>()

    override fun listInstances(): List<InstanceSummary> = instances.values.toList()

    override fun createInstance(
        id: String,
        name: String,
        minecraftVersion: String,
        loader: String?
    ): InstanceSummary {
        require(id.isNotBlank()) { "Instance ID cannot be empty" }
        require(name.isNotBlank()) { "Instance name cannot be empty" }
        require(minecraftVersion.isNotBlank()) { "Minecraft version cannot be empty" }
        check(!instances.containsKey(id)) { "Instance already exists: $id" }

        return InstanceSummary(id, name, minecraftVersion, loader).also {
            instances[id] = it
        }
    }

    override fun deleteInstance(id: String) {
        instances.remove(id)
    }

    override fun launch(instanceId: String): LaunchStatus {
        check(instances.containsKey(instanceId)) { "Instance not found: $instanceId" }
        return native.launch(instanceId)
    }
}


interface NativeLauncherBridge {
    fun launch(instanceId: String): LaunchStatus
}

class JniNativeLauncherBridge : NativeLauncherBridge {
    override fun launch(instanceId: String): LaunchStatus = when (nativeLaunch(instanceId)) {
        1 -> LaunchStatus.Preparing
        2 -> LaunchStatus.Launching
        3 -> LaunchStatus.Running
        else -> LaunchStatus.Failed
    }

    private external fun nativeLaunch(instanceId: String): Int

    companion object {
        init { System.loadLibrary("a_launcher") }
    }
}
