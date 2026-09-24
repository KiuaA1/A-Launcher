package com.kiuaa1.alauncher

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp

private val RefPanel = Color(0xD9161A24)
private val RefPanel2 = Color(0xB9161A24)
private val RefText = Color.White
private val RefMuted = Color(0xFF9DA3B0)
private val RefAccent = Color(0xFF7C4DFF)

@Composable
fun AccountsReferenceScreen() {
    ReferencePage("Who is playing?", "Pick who this launcher should identify you as. You can add accounts, switch between them, or continue locally.") {
        ReferenceRow("Microsoft account", "Premium • Full online play", "2")
        ReferenceRow("ely.by account", "Free • skins & capes, enabled servers", "2")
        ReferenceRow("Local profile", "Offline • pick any name, no password", "3")
        Spacer(Modifier.height(12.dp))
        Button(onClick = {}, colors = ButtonDefaults.buttonColors(containerColor = RefAccent)) { Text("Add account") }
    }
}

@Composable
fun BrowseResourcesReferenceScreen() {
    ReferencePage("Browse Resources", "Mods, resource packs and shaders for your instances.") {
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            TextButton(onClick = {}) { Text("Mods", color = RefText) }
            TextButton(onClick = {}) { Text("Resource Packs", color = RefMuted) }
            TextButton(onClick = {}) { Text("Shaders", color = RefMuted) }
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            ResourceCard("Fabric API", "Lightweight and modular API providing common hooks.")
            ResourceCard("Sodium", "High-performance rendering engine replacement.")
        }
    }
}

@Composable
fun AdvancedReferenceScreen() {
    ReferencePage("Advanced", "Everything else — only if you know what you are doing.") {
        OutlinedTextField("", {}, label = { Text("Search settings") }, modifier = Modifier.fillMaxWidth())
        Spacer(Modifier.height(12.dp))
        ReferenceRow("Allow microphone", "Click to request the notification permission.", "OFF")
        ReferenceRow("Animation Amplitude", "How far cards, rows and pages travel while they animate.", "5")
        ReferenceRow("Wipe Controller Map", "Show custom controller configuration", "›")
        ReferenceRow("Enable shader dumping", "Log command output into the log file.", "OFF")
        ReferenceRow("Experimental JVM tuning", "Lets the performance engine apply additional heap headroom.", "OFF")
    }
}

@Composable
fun GameReferenceScreen() {
    ReferencePage("Game", "RAM, performance mode, Java") {
        ReferenceRow("Quick Device Optimizer", "1-click auto tuning for RAM, resolution, vsync & FPS.", "›")
        ReferenceRow("4GB RAM Phone (Budget / Low-End)", "Recommended for lower-memory devices.", "APPLY")
        ReferenceRow("6GB RAM Phone (Balanced / Mid-Range)", "Balanced memory profile.", "APPLY")
        ReferenceRow("8GB+ RAM Phone (Flagship / Ultra)", "Maximum performance profile.", "APPLY")
        ReferenceRow("RAM for Minecraft", "Allocated heap for Minecraft and mods.", "1024 MB")
    }
}

@Composable
fun SkinStudioReferenceScreen() {
    ReferencePage("Skin Studio", "Edit • Export • Equip skin") {
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            Card(colors = CardDefaults.cardColors(containerColor = RefPanel2), modifier = Modifier.weight(1f)) {
                Box(Modifier.fillMaxWidth().height(260.dp), contentAlignment = Alignment.Center) {
                    Text("Minecraft Skin Preview", color = RefMuted)
                }
            }
            Column(Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(10.dp)) {
                ReferenceRow("Current Skin", "Slot 1 • active", "ACTIVE")
                ReferenceRow("Skin Slot 2", "Empty", "ADD")
                ReferenceRow("Skin Slot 3", "Empty", "ADD")
                Button(onClick = {}, colors = ButtonDefaults.buttonColors(containerColor = RefAccent)) { Text("Import Skin") }
            }
        }
    }
}

@Composable
private fun ReferencePage(title: String, subtitle: String, content: @Composable ColumnScope.() -> Unit) {
    Column(
        Modifier.fillMaxSize().padding(start = 145.dp, top = 30.dp, end = 30.dp, bottom = 30.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Text(title, color = RefText, style = MaterialTheme.typography.headlineLarge)
        Text(subtitle, color = RefMuted)
        content()
    }
}

@Composable
private fun ReferenceRow(title: String, subtitle: String, action: String) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = RefPanel),
        shape = RoundedCornerShape(14.dp)
    ) {
        Row(Modifier.fillMaxWidth().padding(16.dp), verticalAlignment = Alignment.CenterVertically) {
            Column(Modifier.weight(1f)) {
                Text(title, color = RefText)
                Text(subtitle, color = RefMuted, style = MaterialTheme.typography.bodySmall)
            }
            Text(action, color = RefText, style = MaterialTheme.typography.labelMedium)
        }
    }
}

@Composable
private fun ResourceCard(title: String, description: String) {
    Card(
        modifier = Modifier.width(320.dp),
        colors = CardDefaults.cardColors(containerColor = RefPanel),
        shape = RoundedCornerShape(14.dp)
    ) {
        Column(Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Box(Modifier.fillMaxWidth().height(110.dp).background(Color(0xFF2A2C31), RoundedCornerShape(10.dp)))
            Text(title, color = RefText, style = MaterialTheme.typography.titleMedium)
            Text(description, color = RefMuted, style = MaterialTheme.typography.bodySmall)
            Button(onClick = {}) { Text("INSTALL") }
        }
    }
}
