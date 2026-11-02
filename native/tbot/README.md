# T-Bot — Horizon native mod

This is the native-mod implementation track for the existing Horizon Combat Utility project. It produces an `.hnmod` bundle, not a sandboxed `.hmod`.

## Build model

Horizon native mods are mixin bundles compiled into a game build by the Horizon launcher. They are not sandboxed and are pinned to Horizon's native hook API/revision. The current compatibility target is Minecraft 26.3 / Horizon hook API 359.

## Current milestone

The first commit establishes the native bundle structure and a persistent settings model for the requested T-Bot controls. It registers the module in Options → Mods and exposes its settings through Horizon's native settings page.

**This milestone does not yet automate attacks.** The client-side target/attack integration and a dedicated Right-Shift Orchard-style menu are separate implementation steps and must be validated against the pinned Horizon source before being advertised as working.

The built-in native settings page currently exposes button rows, not arbitrary slider/dropdown widgets. The requested full menu will need its own screen integration rather than pretending those controls already exist.

## Requested settings

- Bind and module reset
- Target mode
- Range variability and range
- Sword and axe cooldown windows
- Reaction time
- Miss chance and miss range
- Smart crits and crit delay
- Reduce sweeps and mode
- Ignore shields, piercing, players-only, invisible players
- Weapon-only
- Holding-bind-only and hold bind

## Build

Use the Horizon native-mod tooling from the matching `horizon-rust/horizon-mods` checkout:

```sh
python3 native/tools/native.py check tbot
python3 native/tools/native.py package tbot
```

A package build requires the mod to be added to the native-mod registry metadata in the Horizon mods repository.
