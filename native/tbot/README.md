# T-Bot 0.4.0 — Horizon native triggerbot

T-Bot is a native Horizon .hnmod for Minecraft 26.3. It targets Horizon commit 5e391eb67392941734df7ddd648728c4ed5302f6 and native hook API 2, matching the Horizon build that reported API 2.

## Features

- Crosshair triggerbot: attacks the mob or player currently under the crosshair using Horizon's normal attack implementation.
- Hold-to-activate: hold the configured key while in a world; release it to stop automated attacks.
- Native keybind: the activation key is registered in Horizon's Mods settings and defaults to Right Alt.
- Module enable/disable: uses Horizon's normal Mods toggle.
- Target mode: Anyone, Players, or Mobs.
- Maximum range: 1.5, 2.0, 2.5, or 3.0 blocks.
- Minimum attack charge: 50% through 100%.
- Hold requirement: can be disabled in the settings page.

The settings page contains only controls connected to the trigger logic. Settings are persisted in Horizon's native mod-data storage and cached so the gameplay tick does not repeatedly read files.

## Build validation

CI formats and lints the Rust source, runs trigger-logic/settings tests, applies the mixins to the pinned Horizon source, checks the staged game, builds the complete Horizon client and control binary, packages the .hnmod, and validates its manifest and contents.

A successful build verifies compilation and package structure, not live gameplay. The final release still requires a real launch test in Horizon 5e391eb in a private singleplayer world or a server where automation is explicitly permitted.
