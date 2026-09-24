package com.kiuaa1.alauncher

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

@Composable
fun RuntimeSetupReferenceScreen(onDone: () -> Unit) {
    var installed by remember { mutableStateOf(false) }

    Box(Modifier.fillMaxSize().background(Color(0xFF02030A))) {
        Column(
            Modifier.fillMaxSize().padding(start = 102.dp, top = 28.dp, end = 42.dp, bottom = 28.dp),
            verticalArrangement = Arrangement.SpaceBetween
        ) {
            Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
                Column(Modifier.weight(1f)) {
                    Text(if (installed) "Java is ready" else "Setting up Java", color = Color.White, style = MaterialTheme.typography.headlineLarge, fontWeight = FontWeight.SemiBold)
                    Spacer(Modifier.height(8.dp))
                    Text(
                        if (installed) "Every runtime you picked is installed. Minecraft will use the right one automatically."
                        else "Keep the app open — this takes a minute or two on a normal connection.",
                        color = Color(0xFFB9BDC8), style = MaterialTheme.typography.titleMedium
                    )
                }
                Text(
                    if (installed) "ALL SET" else "INSTALLING",
                    color = Color(0xFFB9C0D0),
                    style = MaterialTheme.typography.labelLarge,
                    modifier = Modifier.clip(RoundedCornerShape(18.dp)).background(Color(0x331D2432)).padding(horizontal = 16.dp, vertical = 9.dp)
                )
            }

            Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(28.dp)) {
                Card(
                    modifier = Modifier.width(430.dp).height(250.dp),
                    shape = RoundedCornerShape(30.dp),
                    colors = CardDefaults.cardColors(containerColor = Color(0xCC151922))
                ) {
                    Column(Modifier.fillMaxSize().padding(30.dp), verticalArrangement = Arrangement.Center) {
                        Text(if (installed) "Setup complete" else "Downloading Java 25", color = Color.White, style = MaterialTheme.typography.headlineSmall, fontWeight = FontWeight.Medium)
                        Spacer(Modifier.height(8.dp))
                        Text(if (installed) "All runtimes installed and verified. You are ready to play." else "Contacting download server...", color = Color(0xFFB9BDC8), style = MaterialTheme.typography.bodyLarge)
                        Spacer(Modifier.height(20.dp))
                        Text(
                            if (installed) "116 / 116 MB     5.7 MB/s     ~0s" else "90 / 116 MB     6.2 MB/s     ~9s",
                            color = Color.White,
                            modifier = Modifier.clip(RoundedCornerShape(18.dp)).background(Color(0x33242B3A)).padding(horizontal = 14.dp, vertical = 8.dp)
                        )
                    }
                }

                Column(Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(18.dp)) {
                    JavaRuntimeCard(8, "Minecraft 1.16.5 and older", "READY", 1f)
                    JavaRuntimeCard(17, "Minecraft 1.17 – 1.20.4", "READY", 1f)
                    JavaRuntimeCard(21, "Minecraft 1.20.5 and newer", if (installed) "INSTALLED" else "DOWNLOADING", if (installed) 1f else .71f)
                    JavaRuntimeCard(25, "Latest JVM — upcoming releases", "INSTALLED", 1f)
                }
            }

            Button(
                onClick = { if (installed) onDone() else installed = true },
                modifier = Modifier.align(Alignment.End).width(305.dp).height(68.dp),
                shape = RoundedCornerShape(36.dp),
                colors = ButtonDefaults.buttonColors(containerColor = if (installed) Color.White else Color(0xFF41495C), contentColor = Color(0xFF0A0B10))
            ) {
                Text(if (installed) "Start Playing" else "Installing...", fontWeight = FontWeight.Medium, style = MaterialTheme.typography.titleLarge)
            }
        }
    }
}

@Composable
private fun JavaRuntimeCard(major: Int, description: String, state: String, progress: Float) {
    Card(
        modifier = Modifier.fillMaxWidth().height(126.dp),
        shape = RoundedCornerShape(28.dp),
        colors = CardDefaults.cardColors(containerColor = Color(0xCC151922))
    ) {
        Row(Modifier.fillMaxSize().padding(18.dp), verticalAlignment = Alignment.CenterVertically) {
            Box(Modifier.size(64.dp).clip(RoundedCornerShape(20.dp)).background(Color(0x331E2432)), contentAlignment = Alignment.Center) {
                Text(major.toString(), color = Color.White, style = MaterialTheme.typography.headlineSmall)
            }
            Spacer(Modifier.width(18.dp))
            Column(Modifier.weight(1f)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text("Java $major", color = Color.White, style = MaterialTheme.typography.titleLarge, fontWeight = FontWeight.Medium)
                    Spacer(Modifier.width(10.dp))
                    Text(state, color = Color(0xFF11131A), style = MaterialTheme.typography.labelMedium, modifier = Modifier.clip(RoundedCornerShape(14.dp)).background(Color(0xFFD9DEE8)).padding(horizontal = 10.dp, vertical = 5.dp))
                }
                Text(description, color = Color(0xFFB9BDC8), style = MaterialTheme.typography.bodyMedium)
                Text("arm64  ·  ~" + when (major) { 8 -> 45; 17 -> 55; 21 -> 60; else -> 65 } + " MB", color = Color(0xFFB9BDC8), style = MaterialTheme.typography.bodySmall)
                Spacer(Modifier.height(10.dp))
                LinearProgressIndicator(
                    progress = { progress },
                    modifier = Modifier.fillMaxWidth().height(6.dp).clip(RoundedCornerShape(4.dp)),
                    color = Color(0xFFE8EBF2),
                    trackColor = Color(0x33434A5A)
                )
            }
        }
    }
}
