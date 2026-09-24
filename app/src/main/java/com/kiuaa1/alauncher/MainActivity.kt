package com.kiuaa1.alauncher

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

private val destinations = listOf("Home", "Instances", "Store", "Settings")

class MainActivity : ComponentActivity() {
    private val launcherApi: LauncherApi = LocalLauncherApi()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent { MaterialTheme { LauncherShell(launcherApi) } }
    }
}

@Composable
private fun LauncherShell(api: LauncherApi) {
    var selected by remember { mutableIntStateOf(0) }

    Scaffold(
        bottomBar = {
            NavigationBar {
                destinations.forEachIndexed { index, label ->
                    NavigationBarItem(
                        selected = selected == index,
                        onClick = { selected = index },
                        icon = {},
                        label = { Text(label) }
                    )
                }
            }
        }
    ) { padding ->
        when (selected) {
            0 -> HomeScreen(onInstances = { selected = 1 })
            1 -> InstancesScreen(api)
            else -> PlaceholderScreen(destinations[selected])
        }
    }
}

@Composable
private fun HomeScreen(onInstances: () -> Unit) {
    Column(
        modifier = Modifier.fillMaxSize().padding(20.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text("A-Launcher", style = MaterialTheme.typography.headlineLarge)
        Text("Minecraft Java for Android", style = MaterialTheme.typography.bodyLarge)
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(20.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
                Text("No instance selected", style = MaterialTheme.typography.titleLarge)
                Button(onClick = onInstances) { Text("Manage Instances") }
            }
        }
    }
}

@Composable
private fun InstancesScreen(api: LauncherApi) {
    var instances by remember { mutableStateOf(api.listInstances()) }
    var showCreate by remember { mutableStateOf(false) }

    Column(
        modifier = Modifier.fillMaxSize().padding(20.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text("Instances", style = MaterialTheme.typography.headlineMedium)
            Button(onClick = { showCreate = true }) { Text("New") }
        }

        if (showCreate) {
            var id by remember { mutableStateOf("") }
            var name by remember { mutableStateOf("") }
            var version by remember { mutableStateOf("1.21.1") }

            Card(modifier = Modifier.fillMaxWidth()) {
                Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    OutlinedTextField(id, { id = it }, label = { Text("Instance ID") }, modifier = Modifier.fillMaxWidth())
                    OutlinedTextField(name, { name = it }, label = { Text("Name") }, modifier = Modifier.fillMaxWidth())
                    OutlinedTextField(version, { version = it }, label = { Text("Minecraft version") }, modifier = Modifier.fillMaxWidth())
                    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                        Button(onClick = {
                            if (id.isNotBlank() && name.isNotBlank() && version.isNotBlank()) {
                                api.createInstance(id.trim(), name.trim(), version.trim(), null)
                                instances = api.listInstances()
                                showCreate = false
                            }
                        }) { Text("Create") }
                        Button(onClick = { showCreate = false }) { Text("Cancel") }
                    }
                }
            }
        }

        if (instances.isEmpty()) {
            Text("No instances yet.")
        } else {
            instances.forEach { instance ->
                Card(modifier = Modifier.fillMaxWidth()) {
                    Row(modifier = Modifier.fillMaxWidth().padding(16.dp), horizontalArrangement = Arrangement.SpaceBetween) {
                        Column {
                            Text(instance.name, style = MaterialTheme.typography.titleMedium)
                            Text("Minecraft " + instance.minecraftVersion)
                        }
                        Button(onClick = {
                            api.deleteInstance(instance.id)
                            instances = api.listInstances()
                        }) { Text("Delete") }
                    }
                }
            }
        }
    }
}

@Composable
private fun PlaceholderScreen(title: String) {
    Column(modifier = Modifier.fillMaxSize().padding(20.dp)) {
        Text(title, style = MaterialTheme.typography.headlineMedium)
        Text("This section is coming next.")
    }
}
