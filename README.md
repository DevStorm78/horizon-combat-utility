# T-Bot — Horizon native mod

T-Bot is a native Rust mixin bundle for Horizon 26.3. It is built as an
`.hnmod` against the pinned Horizon and horizon-mixins revisions in
`compatibility.json`.

## Included

- Right Shift opens and closes the T-Bot settings screen.
- Escape closes the screen; while a bind is being captured, Escape cancels capture.
- Settings are persisted through Horizon's native mod settings store.
- Boolean and enum preferences can be changed from the panel.
- Numeric preferences have adjustable sliders. Left-click the slider to set
  its lower/primary value; right-click to set its upper value. Scalar values
  use either click button.
- The hold-bind row captures keyboard keys and mouse buttons 2–5.
- Reset All Settings restores the defaults; Escape cancels an active bind capture.
- Persisted numbers are clamped and malformed values fall back to safe defaults.

## Important scope

The current public native hooks let this bundle provide a native settings
screen and store preferences, but this project does **not** currently perform
automatic player/entity selection or issue attacks, change inventory slots,
inject movement input, or evade server protections. The combat-related values
are configuration preferences; they are not claimed to drive an automated
combat routine. I will not describe the client as a completed auto-combat bot
when those game-action paths are not implemented.

No anti-cheat evasion, concealment, memory patching, or process injection is
included.

## Build on Linux / CachyOS

Install Rust and the native dependencies:

```bash
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
rustup default 1.99.1
sudo pacman -S --needed base-devel pkgconf alsa-lib systemd-libs
```

The official build script lives in the Horizon native-mods repository. Copy
this bundle into that workspace and run its checks:

```bash
git clone https://github.com/horizon-rust/horizon-mods.git
cp -R native/tbot horizon-mods/native/tbot
cd horizon-mods
python3 native/tools/native.py check tbot
python3 native/tools/native.py package tbot
```

The package is written to `native/target/tbot-0.2.0.hnmod`. The `check`
command downloads the pinned Horizon source, applies T-Bot's mixins, and runs
Cargo's integrated Horizon check. The launcher then performs the full client
build when you install the bundle with `native-build`.

## Install the local native bundle

The packaged `.hnmod` is a Horizon **native mod**, not a Java/Fabric JAR or a
content `.hmod`. Do not put it in the ordinary `mods/` or `horizon-mods/`
content-mod folder.

With Horizon launcher 0.2.23 or later, build and install the local bundle into
Horizon’s isolated Native mods instance:

```bash
horizon-launcher native-build /absolute/path/to/tbot-0.2.0.hnmod --trust-native-code
```

Then select the **Native mods** instance in the launcher and press Play. The
first build downloads the launcher's own Rust toolchain and can take 20–30
minutes; subsequent builds are faster. Native mods run with your account's
computer permissions, so only build source you trust.

## Validation

The GitHub Actions workflow runs Clippy with warnings denied, unit tests,
the Horizon integration check, and `.hnmod` package validation. The launcher
performs the final client build on the computer where you install the bundle.
The workflow does not claim an in-game runtime test.
