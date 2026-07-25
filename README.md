# Custom HUD

Custom HUD (also known as **Beautiful Day Counter**) is a lightweight, client-side HUD for Minecraft. It shows the current survival day, an in-game clock, and a color-shifting bar that follows the day/night cycle without changing gameplay.

![Minecraft](https://img.shields.io/badge/Minecraft-26.1.x-8bc34a?logo=minecraft&logoColor=white)
![Fabric](https://img.shields.io/badge/Loader-Fabric-DBD0B4?logo=fabric)
![NeoForge](https://img.shields.io/badge/Loader-NeoForge-E04E39)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

## Features

- Day counter with localized text.
- 12-hour (AM/PM) or 24-hour clock.
- Smooth day/night progress bar with animated colors.
- Top or bottom placement.
- Independent toggles for the day counter, clock, and day bar.
- Milestone celebration at days 50, 100, 200, 300, 365, and every 100 days afterwards.
- Optional level-up sound at a milestone.
- Configuration screen opened with `H` by default.
- Optional Mod Menu integration on Fabric.
- Works on both Fabric and NeoForge from the same source tree.

## Compatibility

This branch targets Minecraft **26.1.2 and later 26.1.x releases, up to (but not including) 26.2**.

| Loader | Required |
| --- | --- |
| Fabric | Fabric Loader 0.18.6+, Fabric API |
| NeoForge | NeoForge 26.1.2.7-beta+ |
| Java | Java 25 |
| Mod Menu | Optional, Fabric only |

Minecraft 26.1 uses official Mojang mappings and the new GUI extraction API. Builds targeting older Minecraft releases need a separate version branch.

## Installation

1. Install Java 25 and the loader you use.
2. Install Fabric API when using Fabric.
3. Download the matching `customhud-*-26.1.2.jar` from Releases.
4. Drop the JAR into the instance's `mods` folder.
5. Optionally install Mod Menu to access the configuration screen from the mods list.

The mod is client-side. It does not need to be installed on a dedicated server.

## Configuration

Press `H` in-game, or open the mod's configuration entry in Mod Menu (Fabric).

The configuration is saved as `config/customhud.json` and is written atomically so a crash or interrupted write cannot leave a half-written file.

## Building from source

Use the bundled Gradle wrapper with Java 25:

```bash
./gradlew build
```

The loader-specific artifacts are generated in:

- `fabric/build/libs/`
- `neoforge/build/libs/`

Useful development tasks:

```bash
./gradlew :fabric:runClient
./gradlew :neoforge:runClient
```

## Project layout

- `common/` — HUD logic, configuration, screen, translations, and shared resources.
- `fabric/` — Fabric entrypoints, key registration, HUD element registration, and Mod Menu integration.
- `neoforge/` — NeoForge entrypoints, key registration, and GUI events.
- `build-logic/` — shared Gradle conventions for the multi-loader build.

Forge is not included in this branch because the 26.1 toolchain is maintained by Fabric and NeoForge; the old empty `forge/` directory is intentionally excluded from the build.

## License

Released under the [MIT License](LICENSE).

Created by **SCARABOCCHIO100**. Issues and pull requests are welcome on [GitHub](https://github.com/sonoFrangu/CustomHUD).
