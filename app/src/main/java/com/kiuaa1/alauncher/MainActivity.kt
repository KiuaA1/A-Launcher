package com.kiuaa1.alauncher

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import kotlin.math.sin

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

    Box(modifier = Modifier.fillMaxSize().background(Color(0xFF02030B))) {
        when (selected) {
            0 -> HomeScreen(
                onInstances = { selected = 1 },
                onLaunch = { },
            )
            1 -> InstancesScreen(api)
            else -> PlaceholderScreen(destinations[selected])
        }

        Sidebar(
            selected = selected,
            onSelect = { selected = it },
            modifier = Modifier.align(Alignment.CenterStart)
        )
    }
}

@Composable
private fun HomeScreen(
    onInstances: () -> Unit,
    onLaunch: () -> Unit,
) {
    val stars = remember {
        List(90) { index ->
            val x = ((index * 83) % 997) / 997f
            val y = ((index * 47 + 13) % 613) / 613f
            val radius = 0.7f + ((index * 17) % 10) / 8f
            Triple(x, y, radius)
        }
    }

    Box(modifier = Modifier.fillMaxSize()) {
        Canvas(modifier = Modifier.fillMaxSize()) {
            drawRect(
                brush = Brush.verticalGradient(
                    listOf(Color(0xFF01020A), Color(0xFF05091A), Color(0xFF02030B))
                )
            )
            stars.forEachIndexed { index, (x, y, radius) ->
                val twinkle = 0.45f + 0.45f * sin(index.toFloat())
                drawCircle(
                    color = Color.White.copy(alpha = twinkle),
                    radius = radius,
                    center = Offset(size.width * x, size.height * y)
                )
            }
        }

        Column(
            modifier = Modifier.fillMaxSize().padding(start = 132.dp, end = 30.dp, top = 20.dp, bottom = 20.dp),
            verticalArrangement = Arrangement.SpaceBetween
        ) {
            HomeTopBar()
            Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.fillMaxWidth()) {
                AvatarPreview()
                Spacer(Modifier.height(12.dp))
                Button(
                    onClick = onLaunch,
                    modifier = Modifier.width(310.dp).height(74.dp),
                    shape = RoundedCornerShape(38.dp),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = Color.White,
                        contentColor = Color(0xFF090A0F)
                    )
                ) {
                    Text("▶  LAUNCH", fontWeight = FontWeight.Bold, style = MaterialTheme.typography.titleLarge)
                }
                Spacer(Modifier.height(12.dp))
                StatusPill()
            }
            InstanceStrip(onInstances)
        }
    }
}

@Composable
private fun HomeTopBar() {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Box(
                modifier = Modifier.size(58.dp).clip(RoundedCornerShape(16.dp)).background(Color(0xFF08090F))
            ) {
                Text("A", modifier = Modifier.align(Alignment.Center), color = Color.White, fontWeight = FontWeight.Bold, style = MaterialTheme.typography.headlineLarge)
            }
            Spacer(Modifier.width(14.dp))
            Text("A launcher", color = Color.White, style = MaterialTheme.typography.headlineMedium)
        }

        Row(
            modifier = Modifier.clip(RoundedCornerShape(32.dp)).background(Color(0xCC161923)).padding(horizontal = 14.dp, vertical = 9.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Box(modifier = Modifier.size(40.dp).clip(RoundedCornerShape(10.dp)).background(Color(0xFFBFC2C7))) {
                Text("■", modifier = Modifier.align(Alignment.Center), color = Color(0xFF73767D))
            }
            Spacer(Modifier.width(12.dp))
            Text("kiua", color = Color.White, style = MaterialTheme.typography.titleMedium)
            Spacer(Modifier.width(10.dp))
            Text("LOCAL", color = Color.White, modifier = Modifier.clip(RoundedCornerShape(18.dp)).background(Color.Black.copy(alpha = .6f)).padding(horizontal = 12.dp, vertical = 5.dp))
            Spacer(Modifier.width(8.dp))
            Text("⌄", color = Color.White)
        }
    }
}

@Composable
private fun AvatarPreview() {
    Box(modifier = Modifier.height(250.dp).width(180.dp), contentAlignment = Alignment.BottomCenter) {
        Text("kiua", modifier = Modifier.offset(y = (-205).dp).clip(RoundedCornerShape(8.dp)).background(Color(0xDD161923)).padding(horizontal = 14.dp, vertical = 6.dp), color = Color.White)
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Box(modifier = Modifier.size(62.dp).background(Color(0xFFB8BDC4))) {
                Text("::", modifier = Modifier.align(Alignment.Center), color = Color(0xFF4B5058), fontWeight = FontWeight.Bold)
            }
            Row {
                Box(modifier = Modifier.size(42.dp, 76.dp).background(Color(0xFFB51B17)))
                Box(modifier = Modifier.size(42.dp, 76.dp).background(Color(0xFF263B72)))
            }
            Row {
                Box(modifier = Modifier.size(42.dp, 76.dp).background(Color(0xFF101C32)))
                Box(modifier = Modifier.size(42.dp, 76.dp).background(Color(0xFF101C32)))
            }
        }
    }
}

@Composable
private fun StatusPill() {
    Row(
        modifier = Modifier.clip(RoundedCornerShape(28.dp)).background(Color(0xAA171A24)).padding(horizontal = 18.dp, vertical = 10.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text("▣  Java 25", color = Color.White)
        Text("  •  ", color = Color(0xFF8D929E))
        Text("⚙  2304 MB RAM", color = Color.White)
    }
}

@Composable
private fun InstanceStrip(onInstances: () -> Unit) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        InstanceCard("Minecraft...", "▣", onInstances)
        InstanceCard("Fabric 26.3", "◇", onInstances)
        InstanceCard("Fabric 26.3", "◇", onInstances)
        Card(
            onClick = onInstances,
            modifier = Modifier.width(230.dp).height(82.dp),
            shape = RoundedCornerShape(42.dp),
            colors = CardDefaults.cardColors(containerColor = Color(0x99131822)),
        ) {
            Row(modifier = Modifier.fillMaxSize().padding(horizontal = 22.dp), verticalAlignment = Alignment.CenterVertically) {
                Text("+", color = Color.White, style = MaterialTheme.typography.headlineMedium)
                Spacer(Modifier.width(12.dp))
                Text("New Instance", color = Color.White, style = MaterialTheme.typography.titleMedium)
            }
        }
    }
}

@Composable
private fun InstanceCard(name: String, icon: String, onClick: () -> Unit) {
    Card(
        onClick = onClick,
        modifier = Modifier.width(230.dp).height(82.dp),
        shape = RoundedCornerShape(42.dp),
        colors = CardDefaults.cardColors(containerColor = Color(0x99131822))
    ) {
        Row(modifier = Modifier.fillMaxSize().padding(horizontal = 14.dp), verticalAlignment = Alignment.CenterVertically) {
            Text(icon, color = Color.White, style = MaterialTheme.typography.headlineSmall)
            Spacer(Modifier.width(12.dp))
            Text(name, color = Color.White, modifier = Modifier.weight(1f))
            Text("▶", color = Color.White)
            Spacer(Modifier.width(12.dp))
            Text("⋮", color = Color.White)
        }
    }
}

@Composable
private fun Sidebar(selected: Int, onSelect: (Int) -> Unit, modifier: Modifier = Modifier) {
    Column(
        modifier = modifier.padding(start = 34.dp).width(88.dp).clip(RoundedCornerShape(42.dp)).background(Color(0xCC171B26)).padding(vertical = 12.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        listOf("⌂", "➤", "▣", "ⓘ", "☷").forEachIndexed { index, icon ->
            IconButton(onClick = { onSelect(if (index == 0) 0 else if (index == 1) 1 else index.coerceAtMost(3)) }) {
                Text(icon, color = if (selected == index) Color.White else Color(0xFFB8BDC7), style = MaterialTheme.typography.headlineSmall)
            }
        }
    }
}

@Composable
private fun InstancesScreen(api: LauncherApi) {
    var instances by remember { mutableStateOf(api.listInstances()) }
    var showCreate by remember { mutableStateOf(false) }
    Column(modifier = Modifier.fillMaxSize().padding(start = 132.dp, top = 24.dp, end = 30.dp)) {
        Text("Instances", style = MaterialTheme.typography.headlineMedium, color = Color.White)
        Spacer(Modifier.height(16.dp))
        if (showCreate) Text("Create flow coming next.", color = Color.White)
        Button(onClick = { showCreate = true }) { Text("New Instance") }
        instances.forEach { Text(it.name + " • Minecraft " + it.minecraftVersion, color = Color.White) }
    }
}

@Composable
private fun PlaceholderScreen(title: String) {
    Column(modifier = Modifier.fillMaxSize().padding(start = 132.dp, top = 24.dp)) {
        Text(title, style = MaterialTheme.typography.headlineMedium, color = Color.White)
        Text("This section is coming next.", color = Color.White)
    }
}
