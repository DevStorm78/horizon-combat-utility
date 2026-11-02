# T-Bot — Horizon native mod

T-Bot is being developed as a native Horizon `.hnmod` bundle. Native mods are compiled into an isolated Horizon game build; they run with the player's account permissions. This project is intended for singleplayer, private testing, and multiplayer servers that explicitly permit automation. It does not attempt to hide from server-side enforcement.

## Compatibility and build

- Minecraft: 26.3
- Horizon native hook API: 359
- Horizon source tested/pinned: `8dd76f8d48dc606cfa44f3262086584e09f64e4b`
- Horizon Mixins pinned: `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`

The manifest uses named, versioned hooks. The hook catalog in the pinned Horizon source reports API 359, and the pinned Mixins parser supports `hook` + `hook_version`. The native-mod tooling is maintained in the separate `horizon-rust/horizon-mods` repository; this project repository does not contain `native/tools/native.py`.

To validate locally, clone the official native-mod tooling repository, copy this project's `native/tbot` directory into its `native/tbot` directory, then run from that checkout:

```sh
python3 native/tools/native.py check tbot
python3 native/tools/native.py build tbot
python3 native/tools/native.py package tbot
```

A package operation creates an `.hnmod`; it does not prove runtime behaviour. The check/build steps compile against the pinned Horizon source. The launcher registry is separate and requires its own reviewed submission process.

## Current implementation status

- T-Bot registers in Options → Mods.
- Settings are persisted through Horizon's native settings storage.
- The stock settings page is still a temporary button-list UI.
- The custom Right-Shift menu, slider/dropdown widgets, bind capture/reset, and gameplay integration are not complete.
- No attack execution is currently wired. The code must not be described as a working combat bot until target selection, timing, attack dispatch, and in-game behaviour have been implemented and tested.

## Planned settings

- Module bind and reset
- Target mode
- Range variability and range
- Sword and axe cooldown windows
- Reaction time
- Miss chance and miss range
- Smart crits and crit delay
- Reduce sweeps and sweep mode
- Ignore shields and piercing
- Players only and hit invisible players
- Weapon only
- Holding-bind-only and hold bind

## Current implementation boundary\n\nThe UI and settings persistence are implemented, but the module does not yet acquire combat targets or issue automated attacks. No anti-cheat bypass, concealment, or packet manipulation is included. The menu source is pinned to the Horizon revision above and still requires the user's launcher build and in-game validation.\n\n## Performance goals

- No world/entity scan on render frames.
- Tick-driven work only, with early exits while disabled or outside an eligible world.
- Avoid per-tick heap allocations and repeated filesystem reads.
- Cache configuration in memory; persist only on changes.
- Keep UI drawing isolated from gameplay logic.
- Measure release-build tick cost before calling the module multiplayer-optimized.
