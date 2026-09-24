package com.kiuaa1.alauncher

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
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

private data class RuntimeOption(
    val major: Int,
    val description: String,
    val sizeMb: Int,
    val recommended: Boolean
)

private val runtimeOptions = listOf(
    RuntimeOption(8, "Minecraft 1.16.5 and older", 45, false),
    RuntimeOption(17, "Minecraft 1.17 – 1.20.4", 55, true),
    RuntimeOption(21, "Minecraft 1.20.5 and newer", 60, true),
    RuntimeOption(25, "Latest JVM — upcoming releases", 65, false)
)

@Composable
fun JavaRuntimeSelectionScreen(
    onSkip: () -> Unit,
    onInstall: (Set<Int>) -> Unit
) {
    var selected by remember { mutableStateOf(runtimeOptions.map { it.major }.toSet()) }

    Box(Modifier.fillMaxSize().background(Color(0xFF02030A))) {
        Row(
            Modifier.fillMaxSize().padding(start = 102.dp, top = 28.dp, end = 42.dp, bottom = 24.dp),
            horizontalArrangement = Arrangement.spacedBy(46.dp)
        ) {
            Column(
                Modifier.weight(.9f).fillMaxHeight(),
                verticalArrangement = Arrangement.Top
            ) {
                ReferencePill("JAVA RUNTIME ENGINE")
                Spacer(Modifier.height(28.dp))
                Text(
                    "Java for Minecraft",
                    color = Color.White,
                    style = MaterialTheme.typography.headlineLarge,
                    fontWeight = FontWeight.SemiBold
                )
                Spacer(Modifier.height(12.dp))
                Text(
                    "Minecraft requires Java to run on your device. Choose which versions to install — Java 17 and 21 are recommended for modern Minecraft.",
                    color = Color(0xFFB9BDC8),
                    style = MaterialTheme.typography.titleMedium
                )
            }

            Column(
                Modifier.weight(1.45f).fillMaxHeight(),
                verticalArrangement = Arrangement.SpaceBetween
            ) {
                Column(verticalArrangement = Arrangement.spacedBy(18.dp)) {
                    runtimeOptions.forEach { option ->
                        JavaSelectionCard(
                            option = option,
                            selected = option.major in selected,
                            onClick = {
                                selected = if (option.major in selected) {
                                    selected - option.major
                                } else {
                                    selected + option.major
                                }
                            }
                        )
                    }
                }

                Row(
                    Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    TextButton(
                        onClick = onSkip,
                        modifier = Modifier.height(68.dp).width(220.dp)
                    ) {
                        Text(
                            "Skip for now",
                            color = Color(0xFFD5D9E2),
                            style = MaterialTheme.typography.titleLarge
                        )
                    }

                    Spacer(Modifier.width(18.dp))

                    val count = selected.size
                    val totalMb = runtimeOptions.filter { it.major in selected }.sumOf { it.sizeMb }
                    Button(
                        onClick = { if (selected.isNotEmpty()) onInstall(selected) },
                        enabled = selected.isNotEmpty(),
                        modifier = Modifier.height(68.dp).width(445.dp),
                        shape = RoundedCornerShape(36.dp),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = Color.White,
                            contentColor = Color(0xFF0A0B10),
                            disabledContainerColor = Color(0xFF41495C),
                            disabledContentColor = Color(0xFF171A22)
                        )
                    ) {
                        Text(
                            "Install @count runtimes  ·  ~@totalMb MB",
                            fontWeight = FontWeight.Medium,
                            style = MaterialTheme.typography.titleLarge
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun ReferencePill(text: String) {
    Text(
        text,
        color = Color(0xFFB9C0D0),
        style = MaterialTheme.typography.labelLarge,
        fontWeight = FontWeight.SemiBold,
        modifier = Modifier
            .clip(RoundedCornerShape(18.dp))
            .background(Color(0x331D2432))
            .padding(horizontal = 16.dp, vertical = 9.dp)
    )
}

@Composable
private fun JavaSelectionCard(
    option: RuntimeOption,
    selected: Boolean,
    onClick: () -> Unit
) {
    Row(
        Modifier
            .fillMaxWidth()
            .height(158.dp)
            .clip(RoundedCornerShape(30.dp))
            .background(if (selected) Color(0xCC1B202D) else Color(0x99141922))
            .clickable(onClick = onClick)
            .padding(horizontal = 22.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Box(
            Modifier.size(82.dp).clip(RoundedCornerShape(23.dp)).background(Color(0x331E2432)),
            contentAlignment = Alignment.Center
        ) {
            Text(
                "@{option.major}",
                color = Color.White,
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Medium
            )
        }

        Spacer(Modifier.width(24.dp))

        Column(Modifier.weight(1f)) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(
                    "Java @{option.major}",
                    color = Color.White,
                    style = MaterialTheme.typography.headlineSmall,
                    fontWeight = FontWeight.Medium
                )
                if (option.recommended) {
                    Spacer(Modifier.width(12.dp))
                    Text(
                        "RECOMMENDED",
                        color = Color(0xFFC9CED9),
                        style = MaterialTheme.typography.labelMedium,
                        modifier = Modifier
                            .clip(RoundedCornerShape(14.dp))
                            .background(Color(0x331F2533))
                            .padding(horizontal = 11.dp, vertical = 5.dp)
                    )
                }
            }

            Spacer(Modifier.height(5.dp))

            Text(
                option.description,
                color = Color(0xFFB9BDC8),
                style = MaterialTheme.typography.titleMedium
            )

            Spacer(Modifier.height(3.dp))

            Text(
                "arm64  ·  ~@{option.sizeMb} MB",
                color = Color(0xFF858B9A),
                style = MaterialTheme.typography.bodyMedium
            )
        }

        SelectionIndicator(selected)
    }
}

@Composable
private fun SelectionIndicator(selected: Boolean) {
    Box(
        Modifier.size(44.dp).clip(RoundedCornerShape(50)).background(Color(0xFFF1F3F7)),
        contentAlignment = Alignment.Center
    ) {
        Box(
            Modifier.size(18.dp).clip(RoundedCornerShape(50))
                .background(if (selected) Color(0xFF0A0B10) else Color(0xFF8C919C))
        )
    }
}
