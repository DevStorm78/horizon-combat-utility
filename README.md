# T-Bot — Horizon native triggerbot

T-Bot is a native Rust `.hnmod` for Horizon 26.3. Version 0.4.0 targets
Horizon native hook API 2 at commit
`5e391eb67392941734df7ddd648728c4ed5302f6`. It replaces the older
settings-only build with a triggerbot wired into Horizon's real game-input
path.

## Features

- **Crosshair triggerbot:** attacks the living entity currently under the crosshair.
- **Hold-to-activate:** Right Alt by default; release it to stop queuing attacks.
- **Target filter:** Anyone, Players, or Mobs.
- **Reach limit:** 1.5, 2.0, 2.5, or 3.0 blocks.
- **Minimum attack charge:** 50% to 100%.
- **Module toggle:** enable/disable through Horizon's Mods page.
- **Persistent settings:** configuration is saved by Horizon's native mod store.

The trigger uses the client tick, keybind event and Horizon's own attack/input
mechanics. On a connected world it queues a normal attack click, allowing
Horizon to send the server's ordinary attack and swing packets; it does not
pretend to apply local damage to a remote entity.

## Compatibility

- Minecraft: 26.3
- Horizon native hook API: 2
- Horizon revision: `5e391eb67392941734df7ddd648728c4ed5302f6`
- Mod ID: `tbot`
- Bundle: `tbot-0.4.0.hnmod`

This build is not compatible with the older API-359 Horizon build. Use the
Horizon version whose commit identifies as `5e391eb` / API 2.

## Build on Linux / CachyOS

Install the toolchain and native libraries:

```bash
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
rustup default 1.99.1
sudo pacman -S --needed base-devel pkgconf alsa-lib systemd-libs
```

Copy the source into the official Horizon native-mods workspace and build it:

```bash
git clone https://github.com/horizon-rust/horizon-mods.git
cp -R native/tbot horizon-mods/native/tbot
cd horizon-mods
python3 native/tools/native.py check tbot
python3 native/tools/native.py build tbot
python3 native/tools/native.py package tbot
```

The package is written to `native/target/tbot-0.4.0.hnmod`. The GitHub
Actions workflow runs formatting, Clippy, unit tests, the pinned Horizon
integration check, a full Horizon client build, and package-content
validation before uploading the installable bundle.

## Install

A native bundle is **not** a Java `mods/` JAR. Do not put the `.hnmod`
file in the usual `mods/` directory. Use Horizon Launcher's native-build
command to create a separate instance:

```bash
mkdir -p "$HOME/Downloads/tbot-native"
unzip -o tbot-0.4.0.hnmod -d "$HOME/Downloads/tbot-native"
horizon-launcher native-build "$HOME/Downloads/tbot-native" --instance "T-Bot" --trust-native-code
```

Launch the newly created T-Bot instance. Read the source before trusting any
native bundle: it runs with your account's permissions.

## Testing scope

Unit tests cover the activation gates, target filters, range, charge and
settings. A successful CI run proves that the bundle applies to the pinned
Horizon source, compiles in a full client build and packages correctly. It
does **not** by itself prove live combat behaviour in a running game; that
requires a runtime test in Horizon 5e391eb.
