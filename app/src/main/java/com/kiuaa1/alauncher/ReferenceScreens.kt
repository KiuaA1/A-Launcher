package com.kiuaa1.alauncher

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp

private val RefPanel = Color(0xD9161A24)
private val RefPanel2 = Color(0xB9161A24)
private val RefText = Color.White
private val RefMuted = Color(0xFF9DA3B0)
private val RefAccent = Color(0xFF7C4DFF)

@Composable
fun AccountsReferenceScreen() {
    var dialog by remember { mutableStateOf<String?>(null) }

    Box(Modifier.fillMaxSize().background(Color(0xFF071426))) {
        Row(
            Modifier.fillMaxSize().padding(start = 145.dp, top = 30.dp, end = 30.dp, bottom = 30.dp),
            horizontalArrangement = Arrangement.spacedBy(28.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Card(
                Modifier.weight(.9f).height(452.dp),
                colors = CardDefaults.cardColors(containerColor = Color(0xE915171D)),
                shape = RoundedCornerShape(24.dp)
            ) {
                Column(Modifier.padding(34.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Box(
                            Modifier.size(58.dp).clip(RoundedCornerShape(16.dp)).background(Color(0xFF080A0F)),
                            contentAlignment = Alignment.Center
                        ) {
                            Text("ID", color = RefText, style = MaterialTheme.typography.titleLarge)
                        }
                        Spacer(Modifier.width(18.dp))
                        Text("I D E N T I T Y   ·   S I G N   I N", color = RefMuted, style = MaterialTheme.typography.labelLarge)
                    }

                    Spacer(Modifier.height(10.dp))
                    Text("Who is playing?", color = RefText, style = MaterialTheme.typography.headlineLarge)
                    Text(
                        "Pick how this launcher should identify you. You can add more accounts later from the account switcher.",
                        color = RefMuted,
                        style = MaterialTheme.typography.titleMedium
                    )
                    HorizontalDivider(color = Color(0x332F3440))
                    Text("•  Microsoft   — servers · skins · capes", color = RefMuted, style = MaterialTheme.typography.bodyLarge)
                    Text("•  ely.by       — free skins · many servers", color = RefMuted, style = MaterialTheme.typography.bodyLarge)
                    Text("○  Local        — offline / LAN only", color = RefMuted, style = MaterialTheme.typography.bodyLarge)
                }
            }

            Column(
                Modifier.weight(1.1f).height(452.dp),
                verticalArrangement = Arrangement.spacedBy(14.dp)
            ) {
                AccountReferenceOption(
                    title = "Microsoft account",
                    subtitle = "Premium · full online play",
                    badge = "1",
                    selected = true
                ) { dialog = "microsoft" }

                AccountReferenceOption(
                    title = "ely.by account",
                    subtitle = "Free · skins & capes, ely-enabled servers",
                    badge = "2",
                    selected = false
                ) { dialog = "ely" }

                AccountReferenceOption(
                    title = "Local profile",
                    subtitle = "Offline · pick any name, no password",
                    badge = "3",
                    selected = false
                ) { dialog = "local" }

                Spacer(Modifier.height(2.dp))
                Text(
                    "// credentials go straight to the provider — the launcher never sees your password",
                    color = Color(0xFF5F6470),
                    style = MaterialTheme.typography.bodySmall
                )
            }
        }

        when (dialog) {
            "local" -> LocalProfileDialog(onDismiss = { dialog = null })
            "microsoft" -> ProviderDialog("Microsoft account", "Premium · full online play", "CONTINUE") { dialog = null }
            "ely" -> ProviderDialog("ely.by account", "Free · skins & capes, ely-enabled servers", "CONTINUE") { dialog = null }
        }
    }
}

@Composable
private fun AccountReferenceOption(
    title: String,
    subtitle: String,
    badge: String,
    selected: Boolean,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        Modifier.fillMaxWidth().height(104.dp),
        colors = CardDefaults.cardColors(
            containerColor = if (selected) Color(0xFFE8ECF2) else Color(0xD9161A24)
        ),
        shape = RoundedCornerShape(24.dp)
    ) {
        Row(Modifier.fillMaxSize().padding(horizontal = 24.dp), verticalAlignment = Alignment.CenterVertically) {
            Box(
                Modifier.size(60.dp).clip(RoundedCornerShape(17.dp)).background(if (selected) Color(0xFF080A0F) else Color(0xFF080A0F)),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    when {
                        title.startsWith("Microsoft") -> "▦"
                        title.startsWith("ely") -> "◇"
                        else -> "♟"
                    },
                    color = if (selected) Color.White else Color(0xFFAFA5FF),
                    style = MaterialTheme.typography.headlineSmall
                )
            }
            Spacer(Modifier.width(18.dp))
            Column(Modifier.weight(1f)) {
                Text(title, color = if (selected) Color(0xFF17191F) else RefText, style = MaterialTheme.typography.titleLarge)
                Text(subtitle, color = if (selected) Color(0xFF565A64) else RefMuted)
            }
            Text(
                "$badge  →",
                color = if (selected) Color(0xFF17191F) else RefText,
                modifier = Modifier
                    .clip(RoundedCornerShape(15.dp))
                    .background(if (selected) Color(0xFF17191F) else Color.Transparent)
                    .padding(horizontal = 12.dp, vertical = 7.dp)
            )
        }
    }
}

@Composable
private fun LocalProfileDialog(onDismiss: () -> Unit) {
    var name by remember { mutableStateOf("") }
    ReferenceDialog(title = "Mint a local profile", onDismiss = onDismiss) {
        Text("Type the name other players will see. Nothing leaves your device.", color = RefMuted)
        OutlinedTextField(
            value = name,
            onValueChange = { if (it.length <= 16 && it.all { c -> c.isLetterOrDigit() || c == '_' }) name = it },
            placeholder = { Text(">_ player_name", color = Color(0xFF5C606A)) },
            singleLine = true,
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(16.dp)
        )
        Text("○  3 – 16 characters", color = RefMuted)
        Text("○  letters, numbers, underscore", color = RefMuted)
        Text("○  not already on this launcher", color = RefMuted)
        Button(
            onClick = onDismiss,
            enabled = name.length in 3..16,
            modifier = Modifier.fillMaxWidth(),
            colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF284F7E))
        ) { Text("MINT PROFILE") }
    }
}

@Composable
private fun ProviderDialog(title: String, subtitle: String, action: String, onDone: () -> Unit) {
    ReferenceDialog(title = "Sign in", onDismiss = onDone) {
        Text(title, color = RefText, style = MaterialTheme.typography.headlineMedium)
        Text(subtitle, color = RefMuted)
        OutlinedTextField(
            value = "",
            onValueChange = {},
            label = { Text("Email or username") },
            modifier = Modifier.fillMaxWidth()
        )
        Button(
            onClick = onDone,
            modifier = Modifier.fillMaxWidth(),
            colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF4C8E32))
        ) { Text(action) }
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
fun CursorStudioReferenceScreen() {
    var style by remember { mutableIntStateOf(0) }
    var size by remember { mutableFloatStateOf(100f) }
    var opacity by remember { mutableFloatStateOf(100f) }
    var speed by remember { mutableFloatStateOf(1f) }

    Row(
        Modifier.fillMaxSize().padding(start = 145.dp, top = 20.dp, end = 30.dp, bottom = 24.dp),
        horizontalArrangement = Arrangement.spacedBy(14.dp)
    ) {
        Card(
            Modifier.weight(1.25f).fillMaxHeight(),
            colors = CardDefaults.cardColors(containerColor = Color(0xD20F121A)),
            shape = RoundedCornerShape(20.dp)
        ) {
            Column(Modifier.fillMaxSize().padding(18.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text("‹", color = RefMuted, style = MaterialTheme.typography.headlineSmall)
                    Spacer(Modifier.width(8.dp))
                    Text("Cursor Studio", color = RefText, style = MaterialTheme.typography.titleLarge)
                    Spacer(Modifier.weight(1f))
                    Text("♡", color = RefMuted, style = MaterialTheme.typography.titleLarge)
                }
                HorizontalDivider(color = Color(0x332F3440))
                Text("CLASSIC ARROW", color = RefMuted, style = MaterialTheme.typography.labelSmall)
                Box(
                    Modifier.fillMaxWidth().weight(1f).clip(RoundedCornerShape(14.dp)).background(Color(0xFF080A0F)),
                    contentAlignment = Alignment.Center
                ) {
                    Text("➤", color = Color.White, style = MaterialTheme.typography.displayLarge)
                }
                Text("Classic Arrow", color = RefText, style = MaterialTheme.typography.titleMedium)
                Text("by CS Studio  ·  CLASSIC", color = RefMuted, style = MaterialTheme.typography.bodySmall)
            }
        }

        Card(
            Modifier.width(300.dp).fillMaxHeight(),
            colors = CardDefaults.cardColors(containerColor = Color(0xD20F121A)),
            shape = RoundedCornerShape(20.dp)
        ) {
            Column(Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(13.dp)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text("SAVE & APPLY", color = RefText, style = MaterialTheme.typography.labelMedium)
                }
                Text("QUICK STYLE", color = RefMuted, style = MaterialTheme.typography.labelSmall)
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    CursorStyleChip("CLASSIC", 0, style) { style = 0 }
                    CursorStyleChip("GAMEPAD", 1, style) { style = 1 }
                    CursorStyleChip("CUSTOM", 2, style) { style = 2 }
                }
                Text("TUNING", color = RefMuted, style = MaterialTheme.typography.labelSmall)
                CursorSlider("SIZE", size, "%") { size = it }
                CursorSlider("OPACITY", opacity, "%") { opacity = it }
                CursorSlider("ANIMATION SPEED", speed, "x") { speed = it }
                Spacer(Modifier.weight(1f))
                Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                    OutlinedButton(onClick = { size = 100f; opacity = 100f; speed = 1f }, Modifier.weight(1f)) { Text("RESET") }
                    Button(onClick = {}, Modifier.weight(1f)) { Text("EXPORT") }
                }
            }
        }
    }
}

@Composable
private fun CursorStyleChip(title: String, index: Int, selected: Int, onClick: () -> Unit) {
    Card(
        onClick = onClick,
        Modifier.weight(1f).height(66.dp),
        colors = CardDefaults.cardColors(
            containerColor = if (selected == index) Color(0xFF252A34) else Color(0xFF171A22)
        ),
        shape = RoundedCornerShape(12.dp)
    ) {
        Column(Modifier.fillMaxSize(), horizontalAlignment = Alignment.CenterHorizontally, verticalArrangement = Arrangement.Center) {
            Text(if (index == 0) "➤" else if (index == 1) "✛" else "✦", color = RefText)
            Text(title, color = RefMuted, style = MaterialTheme.typography.labelSmall)
        }
    }
}

@Composable
private fun CursorSlider(label: String, value: Float, suffix: String, onChange: (Float) -> Unit) {
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Row {
            Text(label, color = RefMuted, style = MaterialTheme.typography.labelSmall, modifier = Modifier.weight(1f))
            Text("\${value.toInt()}\$suffix", color = RefText, style = MaterialTheme.typography.labelSmall)
        }
        Slider(value = value, onValueChange = onChange, valueRange = if (suffix == "x") .5f..2f else 50f..150f)
    }
}

@Composable
fun InstancesReferenceScreen(api: LauncherApi) {
    var instances by remember { mutableStateOf(api.listInstances()) }
    var showCreate by remember { mutableStateOf(false) }
    if (showCreate) {
        CreateVersionReferenceScreen(onBack = { showCreate = false }, onCreate = { id, name, version, loader ->
            api.createInstance(id, name, version, loader); instances = api.listInstances(); showCreate = false
        })
        return
    }
    Column(Modifier.fillMaxSize().padding(start = 145.dp, top = 24.dp, end = 30.dp, bottom = 28.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Column(Modifier.weight(1f)) {
                Text("Instances", color = RefText, style = MaterialTheme.typography.headlineLarge)
                Text("Manage your Minecraft installations and profiles.", color = RefMuted)
            }
            Button(onClick = { showCreate = true }, shape = RoundedCornerShape(18.dp)) { Text("+  NEW INSTANCE") }
        }
        if (instances.isEmpty()) {
            Card(Modifier.fillMaxWidth().weight(1f), colors = CardDefaults.cardColors(containerColor = RefPanel), shape = RoundedCornerShape(24.dp)) {
                Column(Modifier.fillMaxSize(), horizontalAlignment = Alignment.CenterHorizontally, verticalArrangement = Arrangement.Center) {
                    Text("No instances yet", color = RefText, style = MaterialTheme.typography.headlineSmall)
                    Text("Create an instance to start configuring Minecraft.", color = RefMuted)
                    Spacer(Modifier.height(18.dp))
                    Button(onClick = { showCreate = true }) { Text("CREATE INSTANCE") }
                }
            }
        } else {
            Column(Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(10.dp)) {
                instances.forEach { instance ->
                    Card(Modifier.fillMaxWidth().height(96.dp), colors = CardDefaults.cardColors(containerColor = RefPanel), shape = RoundedCornerShape(18.dp)) {
                        Row(Modifier.fillMaxSize().padding(horizontal = 18.dp), verticalAlignment = Alignment.CenterVertically) {
                            Box(Modifier.size(60.dp).clip(RoundedCornerShape(14.dp)).background(Color(0xFF343841)), contentAlignment = Alignment.Center) {
                                Text(if (instance.loader == null) "▣" else "◇", color = RefText, style = MaterialTheme.typography.headlineSmall)
                            }
                            Spacer(Modifier.width(16.dp))
                            Column(Modifier.weight(1f)) {
                                Text(instance.name, color = RefText, style = MaterialTheme.typography.titleLarge)
                                Text("Minecraft " + instance.minecraftVersion + (instance.loader?.let { "  ·  " + it } ?: "  ·  Vanilla"), color = RefMuted)
                            }
                            OutlinedButton(onClick = {}) { Text("EDIT") }
                            Spacer(Modifier.width(8.dp)); Button(onClick = {}) { Text("PLAY") }
                            Spacer(Modifier.width(8.dp)); Text("⋮", color = RefMuted, style = MaterialTheme.typography.titleLarge)
                        }
                    }
                }
            }
        }
    }
}

@Composable
fun CreateVersionReferenceScreen(onBack: () -> Unit, onCreate: (String, String, String, String?) -> Unit) {
    var loader by remember { mutableStateOf("Forge") }
    var version by remember { mutableStateOf("1.21") }
    var profileName by remember { mutableStateOf("Forge 1.21") }
    var customId by remember { mutableStateOf("forge-1-21") }

    val loaderSubtitle = when (loader) {
        "Fabric" -> "0.16.14 · auto"
        "Forge" -> "1.21-51.0.0"
        "NeoForge" -> "21.1.0 · auto"
        "Quilt" -> "0.24.0 · auto"
        "OptiFine" -> "OptiFine HD U K2 pre1"
        else -> "Vanilla"
    }

    Box(Modifier.fillMaxSize().background(Color(0xFF090A0F))) {
        Column(Modifier.fillMaxSize().padding(start = 145.dp, end = 28.dp), verticalArrangement = Arrangement.spacedBy(14.dp)) {
            Row(Modifier.fillMaxWidth().height(104.dp), verticalAlignment = Alignment.CenterVertically) {
                OutlinedButton(onClick = onBack, modifier = Modifier.size(72.dp), shape = RoundedCornerShape(22.dp), contentPadding = PaddingValues(0.dp)) {
                    Text("‹", color = RefText, style = MaterialTheme.typography.headlineLarge)
                }
                Spacer(Modifier.width(24.dp))
                Column(Modifier.weight(1f)) {
                    Text("Create Version", color = RefText, style = MaterialTheme.typography.headlineLarge)
                    Text("CUSTOM PROFILE ARCHITECT", color = RefMuted, style = MaterialTheme.typography.labelLarge)
                }
                Surface(shape = RoundedCornerShape(24.dp), color = Color.Transparent, border = ButtonDefaults.outlinedButtonBorder) {
                    Text("CUSTOM", color = RefText, modifier = Modifier.padding(horizontal = 24.dp, vertical = 11.dp))
                }
            }

            Card(Modifier.fillMaxWidth(), colors = CardDefaults.cardColors(containerColor = Color(0xD92D2F35)), shape = RoundedCornerShape(28.dp)) {
                Column(Modifier.padding(32.dp), verticalArrangement = Arrangement.spacedBy(18.dp)) {
                    Text("PROFILE IDENTITY", color = RefMuted, style = MaterialTheme.typography.labelLarge)
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Box(Modifier.size(128.dp).clip(RoundedCornerShape(24.dp)).background(Color(0xFF343841)), contentAlignment = Alignment.Center) {
                            Text(when (loader) { "Forge" -> "F"; "Fabric" -> "FAB"; "NeoForge" -> "NF"; "Quilt" -> "Q"; "OptiFine" -> "OF"; else -> "M" }, color = Color.White, fontWeight = FontWeight.Bold, style = MaterialTheme.typography.displaySmall)
                        }
                        Spacer(Modifier.width(28.dp))
                        OutlinedTextField(value = profileName, onValueChange = { profileName = it }, modifier = Modifier.fillMaxWidth(), singleLine = true, textStyle = MaterialTheme.typography.titleLarge, shape = RoundedCornerShape(20.dp))
                    }
                    Text("Tap the tile to choose a profile icon", color = RefMuted)
                }
            }

            Card(Modifier.fillMaxWidth(), colors = CardDefaults.cardColors(containerColor = Color(0xD91F2025)), shape = RoundedCornerShape(28.dp)) {
                Column(Modifier.padding(32.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
                    Text("MOD LOADER ENGINE", color = RefText, style = MaterialTheme.typography.headlineSmall)
                    Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                        listOf("Fabric", "Forge", "NeoForge", "Quilt", "OptiFine").forEach { item ->
                            val selected = loader == item
                            Card(onClick = { loader = item; profileName = "${item} ${version}"; customId = item.lowercase() + "-" + version.replace(".", "-") }, modifier = Modifier.weight(1f).height(58.dp), colors = CardDefaults.cardColors(containerColor = if (selected) Color(0xFFE5E9F0) else Color(0xFF17191F)), shape = RoundedCornerShape(16.dp)) {
                                Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) { Text(item, color = if (selected) Color(0xFF16181E) else RefText) }
                            }
                        }
                    }
                    Text("${loader.uppercase()} LOADER", color = RefMuted, style = MaterialTheme.typography.labelLarge)
                    Surface(Modifier.fillMaxWidth(), color = Color(0xFF111217), shape = RoundedCornerShape(22.dp)) {
                        Row(Modifier.fillMaxWidth().padding(horizontal = 24.dp, vertical = 18.dp), verticalAlignment = Alignment.CenterVertically) {
                            Text(loaderSubtitle, color = RefText, style = MaterialTheme.typography.titleMedium)
                            Spacer(Modifier.weight(1f)); Text("⌄", color = RefMuted, style = MaterialTheme.typography.titleLarge)
                        }
                    }
                    Text("MINECRAFT VERSION", color = RefText, style = MaterialTheme.typography.headlineSmall)
                    Surface(Modifier.fillMaxWidth(), color = Color(0xFF111217), shape = RoundedCornerShape(22.dp)) {
                        Row(Modifier.fillMaxWidth().padding(horizontal = 24.dp, vertical = 18.dp), verticalAlignment = Alignment.CenterVertically) {
                            Text("▣", color = RefText, style = MaterialTheme.typography.titleLarge); Spacer(Modifier.width(14.dp))
                            Text(version, color = RefText, style = MaterialTheme.typography.titleMedium); Spacer(Modifier.weight(1f))
                            Text("⌄", color = RefMuted, style = MaterialTheme.typography.titleLarge)
                        }
                    }
                    Text("Pick a loader build, then a compatible Minecraft version — the profile is created right here.", color = RefMuted)
                }
            }

            Card(Modifier.fillMaxWidth().height(124.dp), colors = CardDefaults.cardColors(containerColor = Color(0xE915171D)), shape = RoundedCornerShape(28.dp)) {
                Row(Modifier.fillMaxSize().padding(horizontal = 30.dp), verticalAlignment = Alignment.CenterVertically) {
                    Column(Modifier.weight(1f)) {
                        Text("Ready to create", color = RefText, style = MaterialTheme.typography.titleLarge)
                        Text("${version}  ·  ${loaderSubtitle}", color = RefMuted)
                    }
                    Button(onClick = { onCreate(customId.ifBlank { "custom-" + version.replace(".", "-") }, profileName.ifBlank { "Minecraft ${version}" }, version, loader.ifBlank { null }) }, modifier = Modifier.width(310.dp).height(72.dp), shape = RoundedCornerShape(40.dp), colors = ButtonDefaults.buttonColors(containerColor = Color(0xFFE7EBF2), contentColor = Color(0xFF17191F))) {
                        Text(if (loader == "OptiFine") "DOWNLOAD & CREATE" else "CREATE", fontWeight = FontWeight.Bold, letterSpacing = androidx.compose.ui.unit.TextUnit(1.5f, androidx.compose.ui.unit.TextUnitType.Sp))
                    }
                }
            }
        }
    }
}
@Composable
private fun CreateInstanceReferenceDialog(
    onDismiss: () -> Unit,
    onCreate: (String, String, String, String?) -> Unit
) {
    var id by remember { mutableStateOf("") }
    var name by remember { mutableStateOf("") }
    var version by remember { mutableStateOf("1.21.8") }
    var loader by remember { mutableStateOf("") }

    ReferenceDialog(title = "New Instance", onDismiss = onDismiss) {
        Text("INSTANCE IDENTITY", color = RefMuted, style = MaterialTheme.typography.labelSmall)
        OutlinedTextField(id, { id = it }, label = { Text("Instance ID") }, modifier = Modifier.fillMaxWidth())
        OutlinedTextField(name, { name = it }, label = { Text("Display name") }, modifier = Modifier.fillMaxWidth())
        OutlinedTextField(version, { version = it }, label = { Text("Minecraft version") }, modifier = Modifier.fillMaxWidth())
        OutlinedTextField(loader, { loader = it }, label = { Text("Loader (optional)") }, modifier = Modifier.fillMaxWidth())
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            OutlinedButton(onClick = onDismiss, Modifier.weight(1f)) { Text("CANCEL") }
            Button(onClick = { if (id.isNotBlank() && name.isNotBlank() && version.isNotBlank()) onCreate(id.trim(), name.trim(), version.trim(), loader.trim().ifBlank { null }) }, Modifier.weight(1f)) { Text("CREATE") }
        }
    }
}

@Composable
fun SettingsReferenceScreen() {
    GameReferenceScreen()
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
    var ram by remember { mutableIntStateOf(1024) }
    var profile by remember { mutableStateOf<String?>(null) }

    Column(
        Modifier.fillMaxSize().padding(start = 100.dp, top = 26.dp, end = 30.dp, bottom = 26.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Box(
                Modifier.size(58.dp).clip(RoundedCornerShape(30.dp))
                    .background(Color(0xFF171A22)),
                contentAlignment = Alignment.Center
            ) {
                Text("‹", color = RefText, style = MaterialTheme.typography.headlineMedium)
            }
            Spacer(Modifier.width(18.dp))
            Column {
                Text("Game", color = RefText, style = MaterialTheme.typography.headlineLarge)
                Text("RAM, performance mode, Java", color = RefMuted, style = MaterialTheme.typography.titleMedium)
            }
        }

        Text("Game", color = RefMuted, style = MaterialTheme.typography.titleLarge,
            modifier = Modifier.padding(start = 12.dp, top = 6.dp))

        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(containerColor = Color(0xD9161921)),
            shape = RoundedCornerShape(24.dp)
        ) {
            Column(Modifier.padding(18.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Box(
                        Modifier.size(52.dp).clip(RoundedCornerShape(15.dp)).background(Color(0xFF090B10)),
                        contentAlignment = Alignment.Center
                    ) { Text("⌘", color = RefText, style = MaterialTheme.typography.headlineSmall) }
                    Spacer(Modifier.width(18.dp))
                    Column(Modifier.weight(1f)) {
                        Text("Quick Device Optimizer", color = RefText, style = MaterialTheme.typography.titleLarge)
                        Text("1-Click auto-tuning for RAM, resolution, renderer, VSync & FSR", color = RefMuted)
                    }
                }

                GameProfileRow("4GB RAM Phone (Budget / Low-End)",
                    "1536 MB RAM · 70% Resolution · GL4ES · 50% FSR · Performance Mode",
                    "4G", profile == "4G") { profile = "4G"; ram = 1536 }

                GameProfileRow("6GB RAM Phone (Balanced / Mid-Range)",
                    "3072 MB RAM · 85% Resolution · MobileGlues · 25% FSR · Balanced",
                    "6G", profile == "6G") { profile = "6G"; ram = 3072 }

                GameProfileRow("8GB+ RAM Phone (Flagship / Ultra)",
                    "4096 MB RAM · 100% Native Res · Zink/MobileGlues · Maximum Mode",
                    "8G+", profile == "8G+") { profile = "8G+"; ram = 4096 }
            }
        }

        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(containerColor = Color(0xD9161921)),
            shape = RoundedCornerShape(0.dp)
        ) {
            Row(
                Modifier.fillMaxWidth().padding(horizontal = 18.dp, vertical = 22.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text("▦", color = RefMuted, style = MaterialTheme.typography.headlineSmall)
                Spacer(Modifier.width(18.dp))
                Column(Modifier.weight(1f)) {
                    Text("RAM for Minecraft", color = RefText, style = MaterialTheme.typography.headlineSmall)
                    Text("Allocated heap for Minecraft and mods", color = RefMuted, style = MaterialTheme.typography.titleMedium)
                }
                Surface(
                    shape = RoundedCornerShape(18.dp),
                    color = Color(0xFF20242E)
                ) {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Text("−", color = RefText, modifier = Modifier.padding(horizontal = 14.dp, vertical = 10.dp))
                        Text("$ram MB", color = RefText, style = MaterialTheme.typography.titleLarge)
                        Text("+", color = RefText, modifier = Modifier.padding(horizontal = 14.dp, vertical = 10.dp))
                    }
                }
            }
        }

        Text("The launcher applies the selected profile to resolution, renderer, VSync, FSR and Java memory.", color = RefMuted, style = MaterialTheme.typography.bodySmall)
    }
}

@Composable
private fun GameProfileRow(
    title: String,
    subtitle: String,
    badge: String,
    selected: Boolean,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth().height(92.dp),
        colors = CardDefaults.cardColors(
            containerColor = if (selected) Color(0xFF242934) else Color(0x071A1D24)
        ),
        shape = RoundedCornerShape(18.dp)
    ) {
        Row(
            Modifier.fillMaxSize().padding(horizontal = 16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Box(
                Modifier.size(54.dp).clip(RoundedCornerShape(14.dp)).background(Color(0xFF0B0D12)),
                contentAlignment = Alignment.Center
            ) { Text(badge, color = RefText, style = MaterialTheme.typography.titleMedium) }
            Spacer(Modifier.width(18.dp))
            Column(Modifier.weight(1f)) {
                Text(title, color = RefText, style = MaterialTheme.typography.titleLarge)
                Text(subtitle, color = RefMuted, style = MaterialTheme.typography.bodyMedium)
            }
            OutlinedButton(
                onClick = onClick,
                shape = RoundedCornerShape(18.dp),
                border = ButtonDefaults.outlinedButtonBorder(enabled = true)
            ) { Text(if (selected) "APPLIED" else "APPLY") }
        }
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
