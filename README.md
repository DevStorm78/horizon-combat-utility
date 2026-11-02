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
command downloads the pinned Horizon source, applies the native bundle and
runs a full Horizon client `cargo check`; `package` then creates the
installable source bundle.

At the pinned Horizon revision, the upstream `native.py build` command has
a package-selection mismatch for the `horizon-ctl` binary. The project's
GitHub Actions workflow applies a small patch only to its temporary copy of
that build script, compiling `horizon-client` and `horizon-ctl` from their
respective packages. It does not modify the upstream Horizon tools.

## Install and run

A native bundle is **not** a Java `mods/` JAR. The `.hnmod` file is a
source bundle that Horizon's launcher validates and compiles into a separate,
isolated game instance. Do not copy the `.hnmod` file into the normal
`mods/` folder.

After a successful CI build, download `tbot-0.2.0.hnmod` from the workflow's
Artifacts section. On Linux, unpack it and ask Horizon Launcher to build it:

```bash
mkdir -p "$HOME/Downloads/tbot-native"
unzip -o tbot-0.2.0.hnmod -d "$HOME/Downloads/tbot-native"
horizon-launcher native-build "$HOME/Downloads/tbot-native" --instance "T-Bot" --trust-native-code
```

This requires Horizon Launcher 0.2.23 or later and a prepared Horizon 26.3
installation. The first native build downloads the launcher's pinned Rust
toolchain and may take a while. Read the source before trusting it: native
mods run with your account's permissions. Launch the newly created **T-Bot**
native-mods instance, not your ordinary instance.

To inspect the CLI options before building, run
`horizon-launcher native-build --help`.

The extracted folder passed to `native-build` must directly contain
`horizon-native.json` and the `mixins/` directory. Keep the `mixins/tbot/`
folder structure intact when extracting the archive.

## Validation

The GitHub Actions workflow runs Clippy with warnings denied,
unit tests, Horizon integration check, full client build, and package
validation. A successful workflow artifact is the installable `.hnmod`.
A local Rust toolchain or a CI run is required to confirm the build; this
README does not claim an in-launcher runtime test.
