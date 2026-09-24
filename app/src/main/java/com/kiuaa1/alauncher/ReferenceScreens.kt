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
    var showPicker by remember { mutableStateOf(false) }
    var showLocal by remember { mutableStateOf(false) }
    var showMicrosoft by remember { mutableStateOf(false) }

    Box(Modifier.fillMaxSize()) {
        Row(
            Modifier.fillMaxSize().padding(start = 145.dp, top = 30.dp, end = 30.dp, bottom = 30.dp),
            horizontalArrangement = Arrangement.spacedBy(28.dp)
        ) {
            Card(
                modifier = Modifier.weight(.9f).fillMaxHeight(.72f),
                colors = CardDefaults.cardColors(containerColor = Color(0xE914151B)),
                shape = RoundedCornerShape(24.dp)
            ) {
                Column(Modifier.padding(34.dp), verticalArrangement = Arrangement.spacedBy(14.dp)) {
                    Text("ID  ·  SIGN IN", color = RefMuted, style = MaterialTheme.typography.labelLarge)
                    Text("Who is playing?", color = RefText, style = MaterialTheme.typography.headlineLarge)
                    Text(
                        "Pick how this launcher should identify you. You can add more accounts later from the account switcher.",
                        color = RefMuted,
                        style = MaterialTheme.typography.titleMedium
                    )
                    HorizontalDivider(color = Color(0x332F3440))
                    Text("•  Microsoft   — servers · skins · capes", color = RefMuted)
                    Text("•  ely.by       — free skins · many servers", color = RefMuted)
                    Text("○  Local        — offline / LAN only", color = RefMuted)
                }
            }

            Column(
                Modifier.weight(1.1f).fillMaxHeight(.72f),
                verticalArrangement = Arrangement.spacedBy(14.dp)
            ) {
                AccountOption(
                    title = "Microsoft account",
                    subtitle = "Premium · full online play",
                    badge = "1",
                    selected = true,
                    onClick = { showMicrosoft = true }
                )
                AccountOption(
                    title = "ely.by account",
                    subtitle = "Free · skins & capes, ely-enabled servers",
                    badge = "2",
                    selected = false,
                    onClick = { showPicker = true }
                )
                AccountOption(
                    title = "Local profile",
                    subtitle = "Offline · pick any name, no password",
                    badge = "3",
                    selected = false,
                    onClick = { showLocal = true }
                )
                Text(
                    "// credentials go straight to the provider — the launcher never sees your password",
                    color = Color(0xFF5F6470),
                    style = MaterialTheme.typography.bodySmall
                )
            }
        }

        if (showPicker) {
            ReferenceDialog(
                title = "Who is playing?",
                onDismiss = { showPicker = false }
            ) {
                AddAccountCard { showPicker = false }
                AccountListRow("kiua", "LOCAL · ACTIVE SKIN SLOT 1", true)
            }
        }

        if (showLocal) {
            ReferenceDialog(
                title = "Mint a local profile",
                onDismiss = { showLocal = false }
            ) {
                Text("Type the name other players will see. Nothing leaves your device.", color = RefMuted)
                OutlinedTextField(
                    value = "",
                    onValueChange = {},
                    placeholder = { Text(">_ player_name", color = Color(0xFF5C606A)) },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp)
                )
                Text("○  3 – 16 characters", color = RefMuted)
                Text("○  letters, numbers, underscore", color = RefMuted)
                Text("○  not already on this launcher", color = RefMuted)
                Button(
                    onClick = { showLocal = false },
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF284F7E))
                ) { Text("MINT PROFILE") }
            }
        }

        if (showMicrosoft) {
            ReferenceDialog(
                title = "Sign in",
                onDismiss = { showMicrosoft = false }
            ) {
                Text("Microsoft", color = RefText, style = MaterialTheme.typography.headlineMedium)
                Text("to continue to Minecraft.", color = RefMuted)
                OutlinedTextField(
                    value = "",
                    onValueChange = {},
                    label = { Text("Email or phone number") },
                    modifier = Modifier.fillMaxWidth()
                )
                Button(
                    onClick = { },
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF4C8E32))
                ) { Text("Next") }
            }
        }
    }
}

@Composable
private fun AccountOption(
    title: String,
    subtitle: String,
    badge: String,
    selected: Boolean,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth().height(96.dp),
        colors = CardDefaults.cardColors(containerColor = if (selected) Color(0xFFE9ECF2) else RefPanel),
        shape = RoundedCornerShape(22.dp)
    ) {
        Row(Modifier.fillMaxSize().padding(horizontal = 24.dp), verticalAlignment = Alignment.CenterVertically) {
            Box(
                Modifier.size(58.dp).clip(RoundedCornerShape(16.dp)).background(if (selected) Color(0xFF111217) else Color(0xFF090A0F)),
                contentAlignment = Alignment.Center
            ) { Text(if (title.startsWith("Microsoft")) "▦" else if (title.startsWith("ely")) "◇" else "♟", color = if (selected) Color.White else Color(0xFFAFA5FF)) }
            Spacer(Modifier.width(18.dp))
            Column(Modifier.weight(1f)) {
                Text(title, color = if (selected) Color(0xFF17191F) else RefText, style = MaterialTheme.typography.titleLarge)
                Text(subtitle, color = if (selected) Color(0xFF565A64) else RefMuted)
            }
            Text("$badge  →", color = if (selected) Color(0xFF17191F) else RefText)
        }
    }
}

@Composable
private fun ReferenceDialog(
    title: String,
    onDismiss: () -> Unit,
    content: @Composable ColumnScope.() -> Unit
) {
    Box(
        Modifier.fillMaxSize().background(Color(0x99000000)),
        contentAlignment = Alignment.Center
    ) {
        Card(
            modifier = Modifier.widthIn(min = 620.dp, max = 920.dp).padding(24.dp),
            colors = CardDefaults.cardColors(containerColor = Color(0xF018191F)),
            shape = RoundedCornerShape(28.dp)
        ) {
            Column(Modifier.padding(24.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text(title, color = RefText, style = MaterialTheme.typography.headlineMedium, modifier = Modifier.weight(1f))
                    TextButton(onClick = onDismiss) { Text("×", color = RefText, style = MaterialTheme.typography.headlineMedium) }
                }
                HorizontalDivider(color = Color(0x332F3440))
                content()
            }
        }
    }
}

@Composable
private fun AddAccountCard(onClick: () -> Unit) {
    Card(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth().height(94.dp),
        colors = CardDefaults.cardColors(containerColor = Color(0x201F222A)),
        shape = RoundedCornerShape(20.dp)
    ) {
        Row(Modifier.fillMaxSize().padding(20.dp), verticalAlignment = Alignment.CenterVertically) {
            Text("+", color = RefText, style = MaterialTheme.typography.displaySmall)
            Spacer(Modifier.width(18.dp))
            Column {
                Text("Add account", color = RefText, style = MaterialTheme.typography.titleLarge)
                Text("CREATE OR SIGN IN", color = RefMuted, style = MaterialTheme.typography.labelMedium)
            }
        }
    }
}

@Composable
private fun AccountListRow(name: String, subtitle: String, active: Boolean) {
    Card(
        modifier = Modifier.fillMaxWidth().height(94.dp),
        colors = CardDefaults.cardColors(containerColor = Color(0xFF1D2028)),
        shape = RoundedCornerShape(20.dp)
    ) {
        Row(Modifier.fillMaxSize().padding(18.dp), verticalAlignment = Alignment.CenterVertically) {
            Box(Modifier.size(58.dp).clip(RoundedCornerShape(15.dp)).background(Color(0xFF111319)), contentAlignment = Alignment.Center) {
                Text("■", color = Color(0xFFC7CBD2))
            }
            Spacer(Modifier.width(16.dp))
            Column(Modifier.weight(1f)) {
                Text(name, color = RefText, style = MaterialTheme.typography.titleLarge)
                Text(subtitle, color = RefMuted, style = MaterialTheme.typography.labelMedium)
            }
            if (active) Text("✓", color = RefText, style = MaterialTheme.typography.headlineSmall)
            Text("⌫", color = RefMuted, modifier = Modifier.padding(start = 18.dp))
        }
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
