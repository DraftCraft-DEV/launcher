# DraftCraft Launcher: plan

DraftCraft Launcher installs, updates and opens **DraftCraft** and **TraceDraw**. It is built from
ArtCraft Launcher (`storytold/craft-launcher`, MIT OR Apache-2.0), whose history this repository
keeps so its contributors keep their credit. This page is the working plan. It goes once the
launcher is done and `README.md` and `AGENTS.md` describe it.

## Decisions (owner, 2026-11-19)

- **Name:** DraftCraft Launcher. Machine names `draftcraft-launcher`, `draftcraft-launcher-*`.
- **App id:** `dev.draftcraft.Launcher` (reverse DNS of draftcraft.dev, which the owner controls).
- **Marks:** DraftCraft™ is a trademark of DraftCraft Engineering Ventures, which lets this
  project use the name and the app icon. ArtCraft's name, logo and "A" mark are ArtCraft's
  trademarks and leave the launcher entirely.
- **Apps:**
  - DraftCraft, from `DraftCraft-DEV/DraftCraft`'s releases, as they are published today.
  - TraceDraw, from the official `tracedraw/tracedraw` releases, unaltered. TraceDraw is
    MIT OR Apache-2.0; the owner approved distributing it on 2026-11-19.
- **Not in the launcher:** every ArtCraft app, and SolidCraft, until its makers agree.
- **No change to DraftCraft or TraceDraw:** the launcher adapts to their releases as they are,
  so DraftCraft's own in-app updater keeps working for current users. How the two updaters share
  the work is decided later, with SkryxDev.

## What the apps publish

| | DraftCraft (`DraftCraft-DEV/DraftCraft`) | TraceDraw (`tracedraw/tracedraw`) |
|---|---|---|
| Version in file names | no: `draftcraft-windows-x64-setup.exe` | yes: `tracedraw-0.48.0-windows-x64-setup.exe` |
| Windows | `-windows-{x64,arm64}-setup.exe`, `-windows-{x64,arm64}.zip` | `-windows-{x64,arm64}-setup.exe` (Inno Setup), `-portable.zip` |
| macOS | `-macos-{arm64,x86_64}.dmg` | `-macos-{arm64,x86_64}.dmg` |
| Linux | `-linux-{x86_64,aarch64}.{AppImage,deb}` | `-linux-{x86_64,arm64}.{AppImage,tar.gz}`, `tracedraw_<v>-1_<arch>.deb` |
| Checksums | `SHA256SUMS.txt` in every release | none; GitHub's own per-file SHA-256 (`digest` in the API) |
| macOS signing | Developer ID only when its Apple secrets are set | ad hoc (`codesign --sign -`) |
| Prereleases | none | a rolling `nightly` prerelease, which the launcher ignores |

## Work

1. **Catalog** (`crates/core/catalog.toml`): two rows, DraftCraft (official) and TraceDraw
   (community). The ArtCraft rows, their icons and their screenshots go.
2. **Release names** (`crates/core/src/platform.rs`):
   - read DraftCraft's names, which carry no version (the version comes from the release tag);
   - add Linux `arm64` beside `aarch64`;
   - add a Windows setup program kind (`-setup.exe`) beside the MSI.
3. **Checksums:** keep the rule that nothing is opened or installed unverified.
   `SHA256SUMS.txt` when the release has one, GitHub's per-file digest otherwise (already read
   by `release.rs`).
4. **Sources:** downloads only from `DraftCraft-DEV/*` and `tracedraw/tracedraw` (pinned per
   app). No other owner.
5. **Off:** the signed hosted catalog on getartcraft.com, the screenshot downloads from
   `storytold/craft-launcher-assets`, the ArtCraft MCP gateway (`craft-ai-mcp`, Settings ›
   AI agents), ArtCraft's social and Discord links.
6. **Windows install:**
   - DraftCraft's and TraceDraw's setup programs run visibly: the user sees the installer, and
     nothing is elevated silently;
   - the portable zip installs without an installer;
   - installed copies are found the way each app's installer registers itself.
7. **macOS:** an app whose DMG has no Developer ID signature is opened when present, not
   installed (the launcher's existing rule). Today that is TraceDraw, and DraftCraft until its
   builds are signed.
8. **Rebrand:** names, app id, `User-Agent: DraftCraft-Launcher/<version>`, Windows registry key
   under `DraftCraft Engineering Ventures`, self-update from `DraftCraft-DEV/launcher`'s
   releases, every translation catalog, packaging (WiX, Info.plist, desktop file, metainfo,
   Flatpak, nfpm), icons (DraftCraft's compass icon; TraceDraw's own icon) with their
   `ATTRIBUTION.md` rows, README, AGENTS.md, NOTICE and brand terms.
9. **Checks:**
   - `cargo xtask ci` (fmt, clippy, tests, wasm);
   - a screenshot of the window;
   - a live check against both apps' current releases: the right file is picked and verified on
     every platform.

## Later

- One updater per app: the launcher and DraftCraft's own updater (decided with SkryxDev).
- SolidCraft, if its makers agree.
- Developer ID signing for DraftCraft's and TraceDraw's macOS builds, so the launcher can install
  them on a Mac.
