# T-Bot 0.4.0 — Horizon native triggerbot

T-Bot is a native Horizon .hnmod for Minecraft 26.3. It targets Horizon commit 5e391eb67392941734df7ddd648728c4ed5302f6 and native hook API 2.

## Features

- Crosshair triggerbot: attacks the mob or player currently under the crosshair using Horizon's normal attack implementation.
- Hold-to-activate: hold the configured key while in a world; release it to stop automated attacks.
- In-game settings menu: press Right Shift while playing. Use Up/Down to select, Left/Right to change values, and Enter to choose or capture a key.
- Configurable held key: defaults to Mouse Button 5 (Forward), the browser-forward side button. Select Activation Key and press Enter, then press a keyboard key or mouse button. Escape cancels capture.
- Module enable/disable: uses Horizon's normal Mods toggle. The in-game settings menu remains available while the module is disabled.
- Target mode: Anyone, Players, or Mobs.
- Maximum range: 1.5, 2.0, 2.5, or 3.0 blocks.
- Minimum attack charge: 50% through 100%.
- Hold requirement: can be disabled in the settings page.

The settings page contains only controls connected to the trigger logic. Settings are persisted in Horizon's native mod-data storage and cached so the gameplay tick does not repeatedly read files. The settings overlay suppresses T-Bot attacks while it is open.

## Build validation

CI formats and lints the Rust source, runs trigger-logic, settings and menu tests, applies the mixins to the pinned Horizon source, checks the staged game, builds the complete Horizon client and control binary, packages the .hnmod, and validates its manifest and contents.

A successful build verifies compilation and package structure, not live gameplay. The final release still requires a real launch test in Horizon 5e391eb in a private singleplayer world or a server where automation is explicitly permitted.

## Compatibility note

This build targets API 2 and uses the stable native HUD and key hooks. It does not include screen-hook injections or make compatibility claims for Horizon revisions without those API 2 hooks.
