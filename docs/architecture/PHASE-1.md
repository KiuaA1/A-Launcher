# A-Launcher Phase 1 — Engine Foundation

## Goal
Establish a stable engine boundary before implementing Minecraft-specific launchers.

## Rules
- Kotlin/Compose owns Android UI and presentation.
- Rust owns launch planning, validation, process orchestration, downloads, runtime management, resolution, logging, and diagnostics.
- Java is used only where Minecraft/JVM integration genuinely requires it.
- UI never constructs a Minecraft command line directly.

## Pipeline
Compose -> A-Launcher API -> Rust Engine -> LaunchPlan -> Runtime/Resolver -> Download/Cache -> Process -> Result/Logs

## Current Rust modules
- launch: LaunchPlan and launch validation
- error: stable engine error model
- fs: Android-safe storage layout abstraction
- cache: SHA-1/size verification
- manifest: Mojang manifest/version JSON models and parsers
- mojang: Mojang metadata resolution
- download: transport abstraction plus verified atomic download flow
- runtime: Java runtime contract
- resolver: version resolution contracts
- instance: instance model
- process: process validation contract

## Download safety
Downloads are written to a temporary `.part` file, verified against Mojang metadata when available, and atomically renamed into place. Existing files are reused only after integrity verification.

The core crate intentionally does not hard-code an HTTP client yet. Android/network-specific code will implement `DownloadTransport`, keeping the engine testable and allowing the transport to evolve independently.

## Repository strategy
The initial implementation lives together while interfaces stabilize. Mature subsystems may later become standalone KIUA repositories without changing the public engine contracts.
