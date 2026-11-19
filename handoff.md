# D.E.V. Launcher: handoff

**For:** Codie, continuing in a local development environment. **From:** Sophie (Claude Code, cloud
session, 2026-11-19), for DraftCraft Engineering Ventures.

Read this first, then `docs/dev-launcher-plan.md` (the decisions and what each app publishes). The
repository is `DraftCraft-DEV/launcher`; all work so far is on the branch **`dev-launcher`**.
There is no `main` yet: create it from this branch once the launcher is reviewed.

## What it is

D.E.V. Launcher, by DraftCraft™ Engineering Ventures, installs, updates and opens **DraftCraft**
(from `DraftCraft-DEV/DraftCraft`'s releases) and **TraceDraw** (from the official
`tracedraw/tracedraw` releases, unaltered, MIT OR Apache-2.0, distribution approved by the owner).
It is built from ArtCraft Launcher (`storytold/craft-launcher`, MIT OR Apache-2.0), whose full
history (649 commits) this repository keeps.

Owner decisions that bind the work:

- **Nothing changes in DraftCraft or TraceDraw.** The launcher adapts to their releases as they are,
  so DraftCraft's own in-app updater keeps working for current users. Sharing updates between the
  two updaters is decided later with SkryxDev.
- **No SolidCraft** until its makers agree. No ArtCraft app, name, logo, server or service.
- Name **D.E.V. Launcher**; machine names `dev-launcher*`; app id **`dev.draftcraft.Launcher`**
  (Flathub verifies it through draftcraft.dev, whose DNS the owner holds).

## Done (commits on `dev-launcher`)

1. **Plan** (`docs/dev-launcher-plan.md`).
2. **Catalog** (`crates/core/catalog.toml`): DraftCraft (official) and TraceDraw (community), each
   pinned to its own repo, icons in `assets/app-icons/`, descriptions in our own words, no
   screenshots.
   - Release names (`crates/core/src/platform.rs`): DraftCraft's names carry no version
     (`draftcraft-windows-x64.zip`: a Windows `.zip` is the portable build); Linux `arm64` is read
     beside `aarch64`. Tests use DraftCraft's real v2.2.0 release
     (`crates/core/tests/fixtures/draftcraft-release.json`) and TraceDraw's names from its release
     workflow.
   - Checksums: DraftCraft has `SHA256SUMS.txt`; TraceDraw has none, so GitHub's per-file digest
     is used (`release.rs` already reads it). Nothing is installed unverified.
   - ArtCraft's hosted catalog (getartcraft.com) and release feed (api.storyteller.ai) are gone:
     GitHub answers for every app, at most hourly.
   - **Test fixture:** the upstream app list is kept as `crates/core/tests/fixtures/test-catalog.toml`
     behind the `test-catalog` feature, turned on only by the native and UI crates' test builds
     (resolver 3 keeps it out of release builds; checked with `cargo tree -e features`). It lets
     their ~650 test references keep exercising former names, Flatpak ids and other publishers.
     Migrating those tests to neutral fixture apps is a nice-to-have, not required.
3. **Rebrand** to D.E.V. Launcher: names, app id, User-Agent `DEV-Launcher/<version>`, Windows
   settings under `DraftCraft Engineering Ventures`, self-update from `DraftCraft-DEV/launcher`,
   links (draftcraft.dev, github.com/DraftCraft-DEV, DraftCraft's Discord), and the **official
   D.E.V. brand** supplied by DraftCraft Engineering Ventures in `assets/brand/` (avatar, mark in
   dark, light and mono, logos, wordmarks; SVG and PNG; use as supplied, never redraw). Every app and
   tray icon is rendered from `dev-avatar.svg` and `dev-mark-mono.svg` by `packaging/icons.sh`
   (needs `resvg`). ArtCraft's logos, marks and screenshots are deleted. The craft-ai-mcp gateway,
   Settings › AI agents and the ArtCraft OCI Flatpak helpers are removed.
4. **Translations:** ArtCraft strings swapped for DraftCraft's in every catalog; DraftCraft's and
   TraceDraw's taglines added in all 13 complete languages; ArtCraft apps' rows removed.

5. **Gate green** (fmt, clippy -D warnings, every test, wasm). Windows portable installs accept an
   exe that reports no version and refuse only a different one. The MSI has its own UpgradeCode and
   component GUIDs, so it can never replace ArtCraft Launcher.
6. **No ArtCraft at runtime:** our own data folders, desktop-file marker (`X-DEV-Launcher=true`),
   version pins (`<data>/versions/versions.json`), install folder `DraftCraft Engineering Ventures`;
   the hosted release-feed URL is deleted (the format stays for the loopback mirror).
7. **Packaging and documents:** the AI gateway is out of every package; Windows installer text,
   folder, links and wizard art; AppStream, desktop file and AppRun marker; macOS DMG background and
   copyright; `AGENTS.md` rewritten for this repo; `NOTICE`, `ATTRIBUTION.md`,
   `assets/brand/LICENSE-brand.txt`, `assets/app-icon/{README.md,LICENSE.txt}`.
8. **Looked at it:** the real app runs and fetches DraftCraft v2.2.0 live from GitHub
   (screenshot sent in the chat). The byline shows "by D.E.V." (the full name did not fit the card).

## Test status

All of `cargo test --workspace` passes, the UI snapshot tests included (with a software Vulkan
driver), except `process::tests::running_paths_are_matched_literally`, which also fails on
untouched upstream in this container (a non-ASCII `/tmp` path). GitHub's CI covers it.

## What is left

1. **CI and releases:** the workflows call `storytold/craft-common-actions`; check they run for
   `DraftCraft-DEV/launcher` (secrets, signing). The release workflow publishes
   `dev-launcher-<ver>-…` assets and `SHA256SUMS.txt`, which the self-update reads.
2. **Owner values:** the Linux package maintainer field is the repo URL; give it a DEV contact
   address if you want one (`packaging/linux/package.sh`). The Flatpak OCI registry is
   `ghcr.io/draftcraft-dev/dev-launcher`.
3. **macOS self-update:** craft-update pins ArtCraft's Developer ID team, so the launcher refuses to
   update itself on a Mac (safe, but it doesn't update). Needs a DEV Developer ID and a craft-update
   option for the team.
4. **Live check:** install DraftCraft and TraceDraw on Windows (portable zip), Linux (AppImage) and
   macOS (opened if present). TraceDraw could not be checked live here: this container's proxy
   refused its GitHub pages.
5. Code comments and test data still name ArtCraft apps (the test fixture catalog keeps upstream's
   apps on purpose). Users never see them.
6. **Header and icon ideas** for the owner's designer: the official lockup (`dev-wordmark*.svg`) in
   the header; a rounded or tighter-cropped icon for macOS and the 16–32 px sizes.

## Later (owner decisions)

- One updater per app: the launcher and DraftCraft's own (with SkryxDev). Also the publisher name
  DraftCraft's installer registers ("Skryx Dev" today), which the launcher matches.
- Running DraftCraft's and TraceDraw's `setup.exe` from the launcher (visibly, never silently
  elevated) instead of portable zips.
- Developer ID signing for both apps' macOS builds, so the launcher can install them.
- SolidCraft, if its makers agree.

## Local setup

- Rust (the toolchain in `rust-toolchain.toml` if present, else stable 1.95+), and
  `rustup target add wasm32-unknown-unknown` for `cargo xtask wasm`.
- `cargo install resvg` only to re-render icons (`packaging/icons.sh`).
- `cargo xtask ci` before every commit. Work on a branch; `main` changes through pull requests the
  owner reviews.
