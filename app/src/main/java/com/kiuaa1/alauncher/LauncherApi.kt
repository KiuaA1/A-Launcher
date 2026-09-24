package com.kiuaa1.alauncher

data class InstanceSummary(
    val id: String,
    val name: String,
    val minecraftVersion: String,
    val loader: String?
)

interface LauncherApi {
    fun listInstances(): List<InstanceSummary>
    fun createInstance(
        id: String,
        name: String,
        minecraftVersion: String,
        loader: String?
    ): InstanceSummary
    fun deleteInstance(id: String)
}

class LocalLauncherApi : LauncherApi {
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
}
