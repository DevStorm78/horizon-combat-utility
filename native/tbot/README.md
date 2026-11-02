# T-Bot — Horizon native mod

This is the native-mod implementation track for the Horizon Combat Utility project. It produces an `.hnmod` bundle, not a sandboxed `.hmod`.

## Build model

Horizon native mods are mixin bundles compiled into a game build by the Horizon launcher. This bundle uses Horizon's **named, versioned native hook points** rather than hard-coded Rust function targets. The current verified compatibility target is Minecraft 26.3, native hook API 359, tested against Horizon revision `8dd76f8d48dc606cfa44f3262086584e09f64e4b`.

The launcher can build hook-based bundles on later Horizon revisions as long as the required hook API versions remain available. The pinned revision records the source used for validation; it is not an anti-cheat or concealment mechanism.

## Current milestone

The bundle registers T-Bot in Options → Mods and exposes a persistent settings model through Horizon's built-in native settings page. That page currently consists of button rows. It is a temporary settings interface, not the requested Orchard-style menu.

**Attack execution is not implemented.** The current code does not yet identify client-side targets or issue attacks. The dedicated Right-Shift menu, real sliders/dropdowns, keybind capture/reset, and attack integration remain future milestones and must be validated against Horizon's actual client APIs.

## Requested settings

- Bind / Reset Module
- Target Mode
- Range Variability and range
- Sword Cooldown and Axe Cooldown
- Reaction Time
- Miss Chance and Miss Range
- Smart Crits and Crit Delay
- Reduce Sweeps and mode
- Ignore Shields and Piercing
- Players Only and Hit Invisible Players
- Weapon Only
- Holding Bind Only and Hold Bind

## Build

From a checkout of this repository, use the matching Horizon native-mod tooling:

```sh
python3 native/tools/native.py check tbot
python3 native/tools/native.py package tbot
```

Packaging creates the `.hnmod` archive. A successful package operation alone does not mean the game compiles or that runtime behaviour works. The mod also needs to be included in the Horizon native-mod registry before the launcher can offer it for installation.
