# A-Launcher Phase 1 — Engine Foundation

## Goal
Establish a stable engine boundary before implementing Minecraft-specific launchers.

## Rules
- Kotlin/Compose owns Android UI and presentation.
- Rust owns launch planning, validation, process orchestration, downloads, runtime management, resolution, logging, and diagnostics.
- Java is used only where Minecraft/JVM integration genuinely requires it.
- UI never constructs a Minecraft command line directly.

## Initial pipeline
Compose -> A-Launcher API -> Rust Engine -> LaunchPlan -> Runtime/Resolver -> Process -> Result/Logs

## Initial Rust modules
- launch: LaunchPlan and launch validation
- error: stable engine error model

Planned modules: runtime, resolver, libraries, assets, natives, instances, downloads, process, logging, diagnostics.

## Repository strategy
The initial implementation lives together while interfaces stabilize. Mature subsystems may later become standalone KIUA-TEAM repositories without changing the public engine contracts.
