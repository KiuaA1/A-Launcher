package com.kiuaa1.alauncher

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import kotlin.math.sin

private val destinations = listOf("Home", "Instances", "Store", "About", "Settings")

private val Space = Color(0xFF02030A)
private val Glass = Color(0xC8171B27)
private val GlassStrong = Color(0xE31A1E2A)
private val Muted = Color(0xFFB9BDC8)

class MainActivity : ComponentActivity() {
    private lateinit var launcherApi: LauncherApi

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        launcherApi = LocalLauncherApi(NativeLauncherBridgeImpl(filesDir.absolutePath))
        setContent { MaterialTheme { LauncherShell(launcherApi) } }
    }
}

@Composable
private fun LauncherShell(api: LauncherApi) {
    var selected by remember { mutableIntStateOf(0) }
    var selectedInstance by remember { mutableStateOf(api.listInstances().firstOrNull()?.id ?: "") }
    var launchStatus by remember { mutableStateOf(LaunchStatus.Idle) }
    var runtimeScreen by remember { mutableIntStateOf(0) }

    Box(Modifier.fillMaxSize().background(Space)) {
        when (runtimeScreen) {
            1 -> JavaRuntimeSelectionScreen(
                onSkip = { runtimeScreen = 0 },
                onInstall = { runtimeScreen = 2 }
            )
            2 -> RuntimeSetupReferenceScreen(onDone = { runtimeScreen = 0 })
            else -> when (selected) {
            0 -> HomeScreen(
                instances = api.listInstances(),
                selectedInstance = selectedInstance,
                onSelectInstance = { selectedInstance = it },
                onInstances = { selected = 1 },
                launchStatus = launchStatus,
                onLaunch = { if (selectedInstance.isNotBlank()) launchStatus = api.launch(selectedInstance) }
            )
            1 -> InstancesReferenceScreen(api)
            2 -> BrowseResourcesReferenceScreen()
            3 -> AccountsReferenceScreen()
            4 -> SettingsReferenceScreen()
                else -> PlaceholderScreen("A-Launcher")
            }
        }

        if (runtimeScreen == 0) {
            ReferenceSidebar(
                selected = selected,
                onSelect = { selected = it },
                modifier = Modifier.align(Alignment.CenterStart)
            )
        }
    }
}

@Composable
private fun HomeScreen(
    instances: List<InstanceSummary>,
    selectedInstance: String,
    onSelectInstance: (String) -> Unit,
    onInstances: () -> Unit,
    launchStatus: LaunchStatus,
    onLaunch: () -> Unit
) {
    Box(Modifier.fillMaxSize()) {
        Starfield()

        Column(
            Modifier.fillMaxSize().padding(start = 145.dp, end = 30.dp, top = 20.dp, bottom = 22.dp),
            verticalArrangement = Arrangement.SpaceBetween
        ) {
            ReferenceTopBar()

            Column(
                Modifier.fillMaxWidth().weight(1f),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center
            ) {
                PixelAvatar()
                Spacer(Modifier.height(2.dp))
                LaunchButton(status = launchStatus, onClick = onLaunch)
                Spacer(Modifier.height(13.dp))
                RuntimePill(onClick = { runtimeScreen = 1 })
            }

            InstanceRail(
                instances = instances,
                selected = selectedInstance,
                onSelect = onSelectInstance,
                onInstances = onInstances
            )
        }
    }
}

@Composable
private fun Starfield() {
    val stars = remember {
        List(150) { i ->
            Triple(
                ((i * 71 + 17) % 1000) / 1000f,
                ((i * 137 + 31) % 1000) / 1000f,
                0.5f + ((i * 13) % 12) / 7f
            )
        }
    }
    Canvas(Modifier.fillMaxSize()) {
        drawRect(
            Brush.verticalGradient(
                listOf(Color(0xFF010208), Color(0xFF070B1B), Color(0xFF010209))
            )
        )
        stars.forEachIndexed { i, (x, y, r) ->
            drawCircle(
                Color.White.copy(alpha = .32f + .38f * ((sin(i.toFloat()) + 1f) / 2f)),
                r,
                Offset(size.width * x, size.height * y)
            )
        }
        // Soft Milky-Way-like band.
        drawCircle(
            Color(0x331C294F),
            size.maxDimension * .62f,
            Offset(size.width * .53f, size.height * .72f)
        )
    }
}

@Composable
private fun ReferenceTopBar() {
    Row(
        Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Box(
                Modifier.size(58.dp).clip(RoundedCornerShape(17.dp)).background(Color(0xFF090A10)),
                contentAlignment = Alignment.Center
            ) {
                Text("A", color = Color.White, fontWeight = FontWeight.Black, style = MaterialTheme.typography.headlineLarge)
            }
            Spacer(Modifier.width(14.dp))
            Text("A launcher", color = Color.White, fontWeight = FontWeight.Medium, style = MaterialTheme.typography.headlineMedium)
        }

        Row(
            Modifier.clip(RoundedCornerShape(30.dp)).background(Glass).padding(horizontal = 13.dp, vertical = 9.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Box(Modifier.size(42.dp).clip(RoundedCornerShape(11.dp)).background(Color(0xFFC8CBD0)), contentAlignment = Alignment.Center) {
                Text("■", color = Color(0xFF737780))
            }
            Spacer(Modifier.width(12.dp))
            Text("kiua", color = Color.White, style = MaterialTheme.typography.titleMedium)
            Spacer(Modifier.width(10.dp))
            Text(
                "LOCAL",
                color = Color.White,
                fontWeight = FontWeight.SemiBold,
                style = MaterialTheme.typography.labelLarge,
                modifier = Modifier.clip(RoundedCornerShape(16.dp)).background(Color.Black.copy(alpha = .62f)).padding(horizontal = 12.dp, vertical = 6.dp)
            )
            Spacer(Modifier.width(8.dp))
            Text("⌄", color = Color.White, style = MaterialTheme.typography.titleLarge)
        }
    }
}

@Composable
private fun PixelAvatar() {
    Box(Modifier.width(190.dp).height(285.dp), contentAlignment = Alignment.BottomCenter) {
        Text(
            "kiua",
            color = Color.White,
            style = MaterialTheme.typography.titleMedium,
            modifier = Modifier
                .offset(y = (-242).dp)
                .clip(RoundedCornerShape(8.dp))
                .background(Color(0xE41A1E29))
                .padding(horizontal = 15.dp, vertical = 7.dp)
        )

        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Box(Modifier.size(70.dp).background(Color(0xFFBFC2C7)), contentAlignment = Alignment.Center) {
                Text("••", color = Color(0xFF50545C), fontWeight = FontWeight.Bold)
            }
            Row {
                Box(Modifier.size(35.dp, 78.dp).background(Color(0xFFC41C19)))
                Box(Modifier.size(35.dp, 78.dp).background(Color(0xFF233E78)))
            }
            Row {
                Box(Modifier.size(35.dp, 76.dp).background(Color(0xFF101B31)))
                Box(Modifier.size(35.dp, 76.dp).background(Color(0xFF101B31)))
            }
        }
    }
}

@Composable
private fun LaunchButton(status: LaunchStatus, onClick: () -> Unit) {
    Button(
        onClick = onClick,
        enabled = status != LaunchStatus.Preparing && status != LaunchStatus.Launching && status != LaunchStatus.Running,
        modifier = Modifier.width(310.dp).height(76.dp),
        shape = RoundedCornerShape(40.dp),
        colors = ButtonDefaults.buttonColors(containerColor = Color.White, contentColor = Color(0xFF08090E)),
        elevation = ButtonDefaults.buttonElevation(defaultElevation = 1.dp)
    ) {
        Text("▶", style = MaterialTheme.typography.titleLarge)
        Spacer(Modifier.width(14.dp))
        Text(when (status) { LaunchStatus.Preparing -> "PREPARING"; LaunchStatus.Launching -> "STARTING"; LaunchStatus.Running -> "RUNNING"; LaunchStatus.Failed -> "RETRY"; LaunchStatus.Idle -> "LAUNCH" }, fontWeight = FontWeight.Bold, letterSpacing = androidx.compose.ui.unit.TextUnit(2f, androidx.compose.ui.unit.TextUnitType.Sp), style = MaterialTheme.typography.titleLarge)
    }
}

@Composable
private fun RuntimePill(onClick: () -> Unit) {
    Row(
        Modifier.clip(RoundedCornerShape(22.dp)).background(Color(0xB91A1E29)).clickable(onClick = onClick).padding(horizontal = 20.dp, vertical = 9.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text("▣", color = Color.White)
        Spacer(Modifier.width(8.dp))
        Text("Java 25", color = Color.White)
        Text("  •  ", color = Muted)
        Text("⚙", color = Color.White)
        Spacer(Modifier.width(7.dp))
        Text("2304 MB RAM", color = Color.White)
    }
}

@Composable
private fun InstanceRail(
    instances: List<InstanceSummary>,
    selected: String,
    onSelect: (String) -> Unit,
    onInstances: () -> Unit
) {
    Row(
        Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(13.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        instances.take(3).forEach { instance ->
            HomeInstanceCard(
                name = instance.name,
                subtitle = "Minecraft " + instance.minecraftVersion,
                icon = if (instance.loader == null) "▣" else "◇",
                selected = selected == instance.id,
                onSelect = { onSelect(instance.id) },
                onInstances = onInstances
            )
        }

        if (instances.isEmpty()) {
            Card(
                onClick = onInstances,
                modifier = Modifier.width(360.dp).height(84.dp),
                shape = RoundedCornerShape(43.dp),
                colors = CardDefaults.cardColors(containerColor = Color(0x91131822))
            ) {
                Row(Modifier.fillMaxSize().padding(horizontal = 22.dp), verticalAlignment = Alignment.CenterVertically) {
                    Text("No instances", color = Color.White, style = MaterialTheme.typography.titleMedium)
                    Spacer(Modifier.width(12.dp))
                    Text("Create your first one", color = Muted)
                }
            }
        }

        Card(
            onClick = onInstances,
            modifier = Modifier.width(238.dp).height(84.dp),
            shape = RoundedCornerShape(43.dp),
            colors = CardDefaults.cardColors(containerColor = Color(0x9B121722))
        ) {
            Row(Modifier.fillMaxSize().padding(horizontal = 22.dp), verticalAlignment = Alignment.CenterVertically) {
                Text("+", color = Color.White, style = MaterialTheme.typography.headlineMedium)
                Spacer(Modifier.width(12.dp))
                Text("New Instance", color = Color.White, style = MaterialTheme.typography.titleMedium)
            }
        }
    }
}
@Composable
private fun HomeInstanceCard(
    name: String,
    subtitle: String,
    icon: String,
    selected: Boolean,
    onSelect: () -> Unit,
    onInstances: () -> Unit
) {
    Card(
        onClick = onSelect,
        modifier = Modifier.width(238.dp).height(84.dp),
        shape = RoundedCornerShape(43.dp),
        colors = CardDefaults.cardColors(
            containerColor = if (selected) Color(0xB21A1E2A) else Color(0x91131822)
        )
    ) {
        Row(Modifier.fillMaxSize().padding(horizontal = 14.dp), verticalAlignment = Alignment.CenterVertically) {
            Box(Modifier.width(5.dp).fillMaxHeight(.62f).clip(RoundedCornerShape(4.dp)).background(if (selected) Color.White else Color.Transparent))
            Spacer(Modifier.width(9.dp))
            Box(Modifier.size(48.dp).clip(RoundedCornerShape(11.dp)).background(Color(0xFF383C43)), contentAlignment = Alignment.Center) {
                Text(icon, color = Color.White, style = MaterialTheme.typography.headlineSmall)
            }
            Spacer(Modifier.width(12.dp))
            Column(modifier = Modifier.weight(1f)) {
                Text(name, color = Color.White, maxLines = 1)
                Text(subtitle, color = Muted, style = MaterialTheme.typography.labelSmall, maxLines = 1)
            }
            IconButton(onClick = onInstances) { Text("⋮", color = Muted, style = MaterialTheme.typography.titleLarge) }
        }
    }
}

@Composable
private fun ReferenceSidebar(selected: Int, onSelect: (Int) -> Unit, modifier: Modifier = Modifier) {
    Column(
        modifier
            .padding(start = 34.dp)
            .width(88.dp)
            .clip(RoundedCornerShape(42.dp))
            .background(Color(0xD5161A25))
            .padding(vertical = 11.dp),
        verticalArrangement = Arrangement.spacedBy(5.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        val icons = listOf("⌂", "➤", "▣", "ⓘ", "☷")
        icons.forEachIndexed { index, icon ->
            val active = selected == index
            Box(
                Modifier.size(70.dp).clip(RoundedCornerShape(35.dp))
                    .background(if (active) Color(0x331F2532) else Color.Transparent)
                    .clickable { onSelect(index.coerceAtMost(4)) },
                contentAlignment = Alignment.Center
            ) {
                Text(icon, color = if (active) Color.White else Muted, style = MaterialTheme.typography.headlineSmall)
            }
        }
    }
}

@Composable
private fun InstancesScreen(api: LauncherApi) {
    var instances by remember { mutableStateOf(api.listInstances()) }
    Column(Modifier.fillMaxSize().padding(start = 145.dp, top = 30.dp, end = 30.dp)) {
        Text("Instances", color = Color.White, style = MaterialTheme.typography.headlineLarge)
        Spacer(Modifier.height(18.dp))
        Button(onClick = { }) { Text("New Instance") }
        Spacer(Modifier.height(14.dp))
        instances.forEach { Text(it.name + "  •  Minecraft " + it.minecraftVersion, color = Color.White) }
    }
}

@Composable
private fun PlaceholderScreen(title: String) {
    Column(Modifier.fillMaxSize().padding(start = 145.dp, top = 30.dp)) {
        Text(title, color = Color.White, style = MaterialTheme.typography.headlineLarge)
        Text("This section is coming next.", color = Muted)
    }
}
