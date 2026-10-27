# Horizon Combat Utility

A minimal Rust/WASM mod for [Horizon](https://github.com/ludovic111/horizon).

## What works

- Minimal HUD with health, hunger, coordinates, held item/slot, and attack/use clicks per second.
- `/combat` commands for status, module preferences, profiles, and HUD visibility.
- Persistent module toggles through the mod storage API.
- Experimental input event polling; no custom textures or animations.

## Combat module support

| Module | Status |
|---|---|
| Triggerbot | **Unsupported action** — no entity targeting or attack-action API |
| W-tap | **Unsupported action** — no movement-input injection API |
| Mace swap | **Unsupported action** — no inventory/held-slot switching API |
| Auto lunge | **Unsupported action** — no attack/item action API |
| Shield breaker | **Unsupported action** — no target/entity interaction API |
| Jump reset | **Unsupported action** — no movement-input injection API |
| Sprint reset | **Unsupported action** — no movement-input injection API |
| Cooldown tracker | **Partial** — click rate and held-item telemetry only; no attack cooldown field |
| Click trainer | **Partial** — live attack/use CPS; no training overlay or target feedback |

Module switches are preferences and status labels. They do not perform the unsupported actions. The public API's experimental input interface reports registered keys, held items, mappings, and clicks per second; it does not expose a general-purpose API for attack, entity selection, inventory switching, or movement-input injection.

No anti-cheat evasion, stealth, memory patching, or process injection is implemented.

## Commands

- `/combat` or `/combat status`
- `/combat help`
- `/combat modules`
- `/combat toggle triggerbot`
- `/combat on wtap` / `/combat off wtap`
- `/combat profile minimal`
- `/combat profile practice`
- `/combat profile custom`
- `/combat coach on` / `/combat coach off` — observes attack/forward timing and counts detected W-tap sequences; it does not press keys for you.
- `/combat hud on` / `/combat hud off`

Names accept hyphens or underscores.

## Build on Linux / CachyOS

Install Rust and add the WASM target:

```bash
rustup target add wasm32-unknown-unknown
cargo install --git https://github.com/ludovic111/horizon cargo-horizon-mod
cargo horizon-mod build
cargo horizon-mod test
```

The expected package is `target/horizon-mod/horizon_combat_utility.hmod`. The build requires network access to fetch Horizon's SDK and toolchain.

Install by copying the generated package into Horizon's mod directory, or use Horizon's **Open Mod Folder** action. The mod declares the experimental `input` capability explicitly in its manifest.

## Build verification

**Verified on 2026-10-27:** [GitHub Actions run #40506576964](https://github.com/DevStorm78/horizon-combat-utility/actions/runs/40506576964) completed successfully. Both **Build .hmod** and **Run sandbox tests** passed, and the workflow uploaded a `.hmod` artifact.

[Download the CI artifact](https://github.com/DevStorm78/horizon-combat-utility/actions/runs/40506576964) from the run's **Artifacts** section. GitHub Actions artifacts expire; this one is scheduled to expire on 2027-01-25. This is a CI artifact, not yet a permanent GitHub Release asset.

GitHub Actions builds the WASM component and runs the mod's sandbox test script on pushes and pull requests.

## Important limitation

This is a functional minimal HUD and input-timing coach, **not a working automated combat client yet**. The current public Horizon mod API does not expose the entity targeting, attack/movement input injection, or inventory switching needed for the requested automation. The module toggles are preferences/status only for those unsupported actions. A true triggerbot, automatic W-tap, mace swap, auto lunge, shield breaker, and jump reset require changes to Horizon's engine/API before this mod can implement them.
