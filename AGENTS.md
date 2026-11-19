# AGENTS.md: guide for agents and contributors

D.E.V. Launcher installs, updates and launches the **D.E.V. App Suite** made and approved by
DraftCraft™ Engineering Ventures: **DraftCraft** (from `DraftCraft-DEV/DraftCraft` releases) and
**TraceDraw** (from the official `tracedraw/tracedraw` releases, unaltered, MIT OR Apache-2.0).
Our mission is to bring the best drafting & graphic design software to everyone!

It is written in **Rust only** (no JavaScript or TypeScript): the desktop app is native
egui/eframe, and the web build is the same Rust compiled to WebAssembly (trunk + wasm-bindgen).
**No Tauri, Electron or webview shells**, ever. It is based on ArtCraft Launcher (MIT OR
Apache-2.0); ArtCraft is credited in `NOTICE` and the README's credits, and the launcher uses no
ArtCraft name, logo, server or service.

The product name is always **D.E.V. Launcher** in user-facing text. Machine names stay lowercase:
crates and binaries (`dev-launcher`, `dev-launcher-*`), file names, and the app id
`dev.draftcraft.Launcher`. The repository is `DraftCraft-DEV/launcher`.

## 1. Workspace map

```text
crates/
  core      dev-launcher-core      pure and wasm-safe: the app catalog (catalog.toml), versions,
                                   GitHub release parsing, picking the right asset per platform
  native    dev-launcher-native    desktop only: HTTP, detecting installed apps, download,
                                   verify and install, self-update
  ui-egui   dev-launcher-ui-egui   the shared egui UI (desktop and web)
apps/
  dev-launcher                     desktop shell (eframe): window, tray icon, notifications
  dev-launcher-web                 the same UI in the browser (trunk; catalog and downloads only)
xtask/                             cargo xtask ci | wasm | version | ico
packaging/                         macOS, Windows, Linux, FreeBSD and web packaging scripts
assets/brand/                      the official D.E.V. brand files (trademarks, used as supplied)
assets/app-icon/                   the launcher's icons, rendered from assets/brand/
assets/app-icons/                  each app's icon, copied from its own repository
assets/fonts/                      Archivo, Inter, Geist Mono (OFL-1.1)
```

Layering: `core` depends on no other workspace crate and must build for `wasm32-unknown-unknown`.
`native` may use `core`. `ui-egui` may use `core` (and `native` only behind
`cfg(not(target_arch = "wasm32"))`). Nothing below the apps opens windows, tray icons or
notifications.

## 2. Golden rules

### Never crash (outranks feature work)

A bad network response, a malformed release, a half-downloaded file, a full disk or a corrupt
settings file must produce an error the user can act on, never a panic.

- **Non-test code never panics.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or
  `unimplemented!`. Return the crate's error type and propagate with `?`.
- Every production crate root carries
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`,
  and every crate manifest `[lints] workspace = true`.
- **No `unsafe`.** The workspace sets `unsafe_code = "forbid"`.
- **Everything from the network is hostile.** Use `get()` instead of indexing, checked arithmetic
  for sizes and offsets, and cap downloads and JSON sizes.
- Every crash fix lands with a test that failed before it.

### Installing other people's software safely

- Download only from the approved apps' own GitHub releases (and GitHub's asset CDN):
  `https://github.com/DraftCraft-DEV/DraftCraft/releases` and
  `https://github.com/tracedraw/tracedraw/releases`. Each app's repo is pinned in
  `crates/core/catalog.toml`. The one other exception is a developer's own loopback mirror
  (`DEV_LAUNCHER_DOWNLOAD_BASE`, `cargo xtask local`), loopback-only and behind the `local-mirror`
  cargo feature, which release builds never enable.
- **Verify every download before running or installing it**: against the release's
  `SHA256SUMS.txt`, or GitHub's own SHA-256 digest of the file where an app publishes no sums
  file. An unverified download is never installed.
- On macOS a DMG installs only when the app's Developer ID team is known (`mac_team`); with none,
  the launcher only opens an app that is already there.
- Never run an installer with elevated rights silently; tell the user what is about to happen.
- Never delete a user's files outside what the launcher itself installed.
- **Agents never add an app.** A new catalog row, download source or install path, or anything
  that loosens the rules above, is the owner's call alone. SolidCraft is not added until its
  makers give permission.
- The launcher's self-update uses its own `craft_update::App` (`crates/native/src/self_update.rs`),
  never `craft_update::suite::LAUNCHER`, which is ArtCraft's.

### Look and feel

Soft rounded corners (`theme::RADIUS`, 6 px, for buttons, inputs, chips and menus;
`theme::CARD_RADIUS`, 10 px, for cards, dialogs and panels; cards stand apart with a gap; the window itself, `theme::WINDOW_RADIUS`, where the app draws its
frame and the window is not maximized; nothing is square), 1 px
hairlines instead of shadows, Archivo for headings and app names, Inter
for body text, upper-case Geist Mono labels and buttons, colours from `theme::Tokens` by name.
The app list sits on a blueprint (`backdrop.rs`: DraftCraft's blueprint blues panning light to dark, a drafting grid that
drifts slowly and stands still with reduce motion). Build new UI from `widgets.rs`; never hard-code a colour. Animations follow the OS's reduce-motion
setting (`crates/ui-egui/src/motion.rs`).

### Languages

Every user-facing string goes through `crates/ui-egui/src/i18n` (`tl!("…")`, `fmt`, `tn`). English
in the code is the lookup key; adding a string means adding its row to every catalog marked
`complete_menus`, which the i18n tests check. Translate from the meaning of the English text.
Product and app names (D.E.V. Launcher, DraftCraft, TraceDraw) stay in English. CJK and Georgian
text use the system's own font; never bundle one.

### Brand, assets and privacy

1. **The D.E.V. brand** (`assets/brand/`) is DraftCraft Engineering Ventures' trademark, not
   MIT/Apache (`assets/brand/LICENSE-brand.txt`): use the files exactly as supplied, never redraw
   or recolour them. Icons are rendered from them by `packaging/icons.sh`; never edit renders by hand.
2. **Every bundled asset has a row in `ATTRIBUTION.md`** (path, title, author, source, license) in
   the same change. Licences: MIT, Apache-2.0, BSD, ISC, OFL-1.1, CC0-1.0, CC-BY(-SA)-4.0, or our
   own trademarks.
3. **Never break wasm.** `cargo xtask wasm` must pass. File-system, process and OS code is
   `cfg(not(target_arch = "wasm32"))`.
4. **Tests are the gate.** Release parsing and asset picking in `core` get unit tests with real
   GitHub release JSON fixtures under `crates/core/tests/fixtures/`.
5. Requests use a generic `DEV-Launcher/<version>` User-Agent. No person's name, email or other
   personal details go into requests, logs or the repository; credit people only by their public
   GitHub handles. No secrets in the repository.
6. Time goes through `jiff`; do not hand-roll calendar conversion or add `chrono` beside it.

### Branches

`draftcraft-launcher` is the default branch and the main line of D.E.V. Launcher; agents push
their work there. `artcraft-launcher` is a record of the ArtCraft Launcher code this project started
from: never push to it, merge into it or delete it. Every push to `draftcraft-launcher` builds test
downloads (`.github/workflows/test-build.yml`, under the run's Artifacts).

## 3. Before you finish a task

```sh
cargo xtask ci              # fmt --check, clippy -D warnings, test, wasm check
```

Or piecemeal: `cargo test -p <crate>`, `cargo clippy -p <crate> --all-targets -- -D warnings`,
`cargo xtask wasm`. If you touched packaging: `shellcheck packaging/*.sh packaging/*/*.sh
packaging/linux/AppRun` and `actionlint`.

## 4. Releasing

The version lives only in `[workspace.package] version` in the root `Cargo.toml`
(`cargo xtask version`, `cargo xtask version set X.Y.Z[-pre]`). Pushing to the `release` branch runs
`.github/workflows/release.yml`, which builds every platform and creates the draft GitHub Release
**D.E.V. Launcher v<version>** with `SHA256SUMS.txt`.

Release assets (keep these names exact; the launcher's own self-update and the README rely on them):

| Platform | Files |
|---|---|
| Windows | `dev-launcher-<ver>-windows-{x64,x86,arm64}.msi`, `…-windows-{x64,x86,arm64}-portable.zip` |
| macOS | `dev-launcher-<ver>-macos-universal.dmg` |
| Linux | `dev-launcher-<ver>-linux-{x86_64,aarch64}.{AppImage,AppImage.zsync,deb,rpm,tar.gz,flatpak}`, `dev-launcher-<ver>-linux-riscv64.{deb,rpm,tar.gz}` |
| FreeBSD | `dev-launcher-<ver>-freebsd-x86_64.tar.gz` |
| Web | `dev-launcher-web-<ver>.zip` |

Packaging facts the code relies on:

- The Windows MSI is **per-user** (no UAC prompt), by default into
  `%LOCALAPPDATA%\Programs\D.E.V. Launcher`. Double-clicked it opens a WixUI setup wizard
  (Welcome; a folder page with a "Create a desktop shortcut" box; Ready; progress; Finish with
  "Launch"), whose art `package.ps1` draws at build time; self-update runs it with `/passive`, no
  wizard. The folder and the shortcut choice are recorded under
  `HKCU\Software\DraftCraft Engineering Ventures\D.E.V. Launcher` (`InstallDir`,
  `DesktopShortcutChoice`) and reused by the next version, and `self_update.rs` reads `InstallDir`
  to know the running copy came from the MSI. Its shortcuts set AppUserModelID
  `dev.draftcraft.Launcher`, which toast notifications need. `portable.txt` beside the exe switches on portable mode
  (`DevLauncherData\` next to the exe).
- The macOS bundle is `D.E.V. Launcher.app` (executable `Contents/MacOS/D.E.V. Launcher`), a
  normal Dock app that can also sit in the menu bar: `LSUIElement` must never be set.
- Linux: the tray uses StatusNotifierItem over D-Bus and notifications use
  `org.freedesktop.Notifications`, both in pure Rust (no GTK, appindicator or libdbus).
- Inside the Flatpak launcher (`process::in_sandbox()`), apps normally install as host Flatpaks
  and run through `flatpak-spawn --host`. When the host has no usable Flatpak — none installed,
  or `flatpak-spawn` not allowed — `linux::order_available_kinds` offers the release tarball
  instead: it is unpacked into `<data>/apps/<app>` (`bin/`, `share/`) and run with `HOME` =
  `<data>/apps/<app>/home`, so an app's settings stay in the sandbox. An update swaps
  `bin/` and `share/` and keeps `home/`; removing the app takes those two folders only. Outside
  a sandbox the tarball is not on offer at all.
- `--socket=pulseaudio` in the launcher manifests is for an app running *inside* the sandbox
  (an app's ALSA reaches the runtime's PulseAudio plugin through it); it also covers the
  microphone.
- `build.rs` reads `DEV_LAUNCHER_BUILD_SHA` / `DEV_LAUNCHER_BUILD_DATE` and, on Windows,
  embeds `assets/app-icon/dev-launcher.ico`, failing the build when
  `DEV_LAUNCHER_REQUIRE_WINRES=1` and it can't.

## 5. Parallel agents

- Use your own target dir (`CARGO_TARGET_DIR=target/agent-<name>`) and edit only the files you own.
- Keep every `Cargo.toml` valid at all times: the `crates/*` and `apps/*` globs mean one broken
  manifest breaks everyone's build. Write manifests to a temp file and `mv` them into place.
- If someone else's in-progress edit breaks the build, wait and retry; don't fix their files.
