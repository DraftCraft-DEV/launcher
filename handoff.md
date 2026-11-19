# D.E.V. Launcher: handoff

**For:** Codie, continuing in a local development environment. **From:** Sophie (Claude Code, cloud
session, 2026-11-19), for DraftCraft Engineering Ventures.

Read this first, then `docs/dev-launcher-plan.md` (the decisions and what each app publishes). The
repository is `DraftCraft-DEV/launcher`; all work so far is on the branch **`draftcraft-launcher`**.
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

## Done (commits on `draftcraft-launcher`)

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
   links (draftcraft.dev, github.com/DraftCraft-DEV, DraftCraft's Discord), its own mark
   (`assets/brand/dev-launcher-mark.svg`; every icon rendered from it by `packaging/icons.sh`, which
   needs `resvg`). ArtCraft's logos, marks and screenshots are deleted. The craft-ai-mcp gateway,
   Settings › AI agents and the ArtCraft OCI Flatpak helpers are removed.
4. **Translations:** ArtCraft strings swapped for DraftCraft's in every catalog; DraftCraft's and
   TraceDraw's taglines added in all 13 complete languages; ArtCraft apps' rows removed.

## Test status at hand-off

- `dev-launcher-core`: all pass.
- `dev-launcher-native`: all pass except two. `process::tests::running_paths_are_matched_literally`
  also fails on untouched upstream (environment: a non-ASCII `/tmp` path), so it is not ours.
  `detect::tests::the_launcher_records_its_folder_under_the_publisher_key` expects the old registry
  path: update its expectation to `SOFTWARE\DraftCraft Engineering Ventures\D.E.V. Launcher`.
- `dev-launcher-ui-egui`: 105 lib tests pass; one fails,
  `install_target::tests::an_update_keeps_the_installed_copys_format_as_the_backend_does`. Not
  investigated yet. The integration tests (`crates/ui-egui/tests/`) were not run after the last
  changes.
- **Not run yet:** `cargo xtask ci` as a whole (fmt, clippy -D warnings, wasm), the desktop app
  itself, a screenshot, and a live install against the real releases.

## What is left, in order

1. **Get the gate green:** `cargo xtask ci`. Fix the two tests above and whatever clippy, fmt and
   the integration tests report. Expect tests in `crates/ui-egui/tests/*.rs` that assert
   ArtCraft-era text or links.
2. **Windows: the version check on portable zips.** After unzipping, `install.rs` compares the
   exe's file version with the release (`detect::windows::executable_version`). DraftCraft's exe
   sets one; check TraceDraw's. If an exe carries none, the install rolls back, so skip the check
   when the exe has no version (keep it when it has one), with a test.
3. **Remaining ArtCraft text** (code comments, docs, packaging):
   `git grep -i -E 'artcraft|storytold|getartcraft|storyteller' -- ':!LICENSE*' ':!NOTICE' ':!Cargo.lock' ':!crates/core/tests/fixtures'`.
   The biggest are `README.md`, `ATTRIBUTION.md`, `AGENTS.md`, `crates/native/src/install.rs`,
   `packages.rs`, `release.rs`, `login.rs` (start at login), `crates/native/src/desktop_entry.rs`
   (desktop file categories per app: add `draftcraft`/`tracedraw`, drop the Craft apps) and the
   language tests (`crates/ui-egui/tests/{german,japanese,...}.rs`).
   - Keep `storytold/craft-libs` (craft-update, craft-flags, the egui fork) and
     `storytold/craft-common-actions` (CI): MIT/Apache dependencies, not branding.
4. **Documents:**
   - `README.md` (what it is, downloads, credit to ArtCraft Launcher);
   - `AGENTS.md` (rewrite for this repository: keep never-crash, verified downloads, no silent
     elevation; this repo's own approved-app rule naming DraftCraft and TraceDraw; drop ArtCraft's
     house rules);
   - `NOTICE` (keep ArtCraft Launcher's copyright and licence; add DraftCraft Engineering Ventures);
   - `ATTRIBUTION.md` (rows for `assets/brand/*`, `assets/app-icon/*` (our mark), `assets/app-icons/draftcraft.png`
     (from DraftCraft-DEV/DraftCraft, the DraftCraft trademark of DraftCraft Engineering Ventures)
     and `tracedraw.png` (from tracedraw/tracedraw at 0fdff91, MIT OR Apache-2.0); remove rows of
     deleted files);
   - `docs/brand/LICENSE-brand.txt` for the D.E.V. mark;
   - `assets/app-icon/README.md` and `LICENSE.txt`.
5. **Packaging:**
   - **WiX:** give `packaging/windows/dev-launcher.wxs` a **new `UpgradeCode`** (and new component
     GUIDs). It still has ArtCraft Launcher's, so our MSI would replace ArtCraft Launcher on a
     user's machine. Must change before any release.
   - **Art and names:** the Windows wizard art in `package.ps1` and the macOS DMG background
     (`packaging/macos/dmg/`) still use the ArtCraft wordmark/mark; draw them from
     `assets/brand/`. Remove `craft-ai-mcp` from `packaging/{linux,freebsd,macos}` and the Flatpak
     manifests. Check `packaging/linux/*.desktop` and metainfo text.
6. **CI and releases:** the workflows call `storytold/craft-common-actions`; check they run for
   `DraftCraft-DEV/launcher` (secrets, signing). The release workflow publishes
   `dev-launcher-<ver>-…` assets and `SHA256SUMS.txt`, which the self-update reads.
7. **Look at it:** `cargo run -p dev-launcher`, screenshot both themes, check the header (mark +
   "D.E.V. LAUNCHER"), the two cards, Settings and About.
8. **Live check:** install DraftCraft and TraceDraw on Windows (portable zip), Linux (AppImage) and
   macOS (opened if present; not installed until their DMGs carry a Developer ID, by design).

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
