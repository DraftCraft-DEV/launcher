# Attribution

Every non-code asset in this repository, with its author, source and license. D.E.V.
Launcher's own code and original assets are MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE) and [`NOTICE`](NOTICE)). When you add an asset, add a row here
in the same change; third-party assets must be permissively licensed and keep their license file
next to them.

## Bundled in the app

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/app-icon/` (all files) | D.E.V. Launcher app and tray icons | DraftCraft Engineering Ventures | Rendered unmodified from `assets/brand/dev-avatar.svg` and `dev-mark-mono.svg` by `packaging/icons.sh` | Not open source; trademark of DraftCraft Engineering Ventures, [`assets/brand/LICENSE-brand.txt`](assets/brand/LICENSE-brand.txt) |
| `assets/brand/` (all files) | D.E.V. avatar, mark, logo and wordmark (SVG and PNG), shown in the window header and installers | DraftCraft Engineering Ventures | Official brand files supplied by the owner, unmodified | Not open source; trademark of DraftCraft Engineering Ventures, [`assets/brand/LICENSE-brand.txt`](assets/brand/LICENSE-brand.txt) |
| `packaging/macos/dmg/` (all files) | macOS DMG window background and layout | D.E.V. Launcher contributors, with the D.E.V. mark (linked, not copied) | Original work, see [`packaging/macos/dmg/README.md`](packaging/macos/dmg/README.md) | MIT OR Apache-2.0 (the mark: [`assets/brand/LICENSE-brand.txt`](assets/brand/LICENSE-brand.txt)) |
| `assets/social/discord.svg`, `github.svg` | Discord and GitHub logos, linking to the DraftCraft community | [Simple Icons](https://simpleicons.org/) 16.34.0 | See [`assets/social/README.md`](assets/social/README.md) | CC0-1.0 (logos are their owners' trademarks) |
| `assets/social/globe.svg` | Globe, linking to draftcraft.dev | [Lucide](https://lucide.dev) 0.468.0 | See [`assets/social/README.md`](assets/social/README.md) | ISC |
| (not bundled) the pictures on each app's details page | Screenshots of the apps, fetched when a page is opened and kept in the launcher's cache folder | The apps' contributors | [DraftCraft-DEV/launcher-assets](https://github.com/DraftCraft-DEV/launcher-assets), pinned to a tag (`SCREENSHOTS_REV` in `crates/core/src/details.rs`); each picture's source and license are listed in that repository's `ATTRIBUTION.md` | Each app's own license for the interface (MIT OR Apache-2.0); the work shown in them is public domain or made in the app, as that file says |
| `assets/fonts/Inter-Variable.ttf` | Inter 4.001, variable (body text) | The Inter Project Authors (Rasmus Andersson) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/inter> (`Inter[opsz,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Inter.txt`](assets/fonts/OFL-Inter.txt) |
| `assets/fonts/Archivo-Variable.ttf` | Archivo 2.001, variable (headings and app names) | The Archivo Project Authors (Omnibus-Type) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/archivo> (`Archivo[wdth,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Archivo.txt`](assets/fonts/OFL-Archivo.txt) |
| `assets/fonts/GeistMono-Variable.ttf` | Geist Mono 1.701, variable (labels and buttons) | The Geist Project Authors (Vercel) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/geistmono> (`GeistMono[wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-GeistMono.txt`](assets/fonts/OFL-GeistMono.txt) |
| (dependency) `epaint_default_fonts/fonts/NotoEmoji-Regular.ttf` | Noto Emoji Regular, monochrome emoji fallback for arbitrary release-note, feed and alert text | The Noto Project Authors (Google) | Bundled transitively by `egui 0.37`'s `monochrome_emoji_fonts` feature from `epaint_default_fonts`; <https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts> | SIL OFL 1.1, `epaint_default_fonts/fonts/OFL.txt`; see also the dependency `README` and egui `README.md` |
| (dependency) `epaint_default_fonts/fonts/emoji-icon-font.ttf` | emoji-icon-font, monochrome symbol/emoji fallback | John Slegers | Bundled transitively by `egui 0.37`'s `monochrome_emoji_fonts` feature from `epaint_default_fonts`; <https://github.com/jslegers/emoji-icon-font> | MIT, Copyright (c) 2014 John Slegers, `epaint_default_fonts/fonts/emoji-icon-font-mit-license.txt` |
| (dependency) `epaint_default_fonts/fonts/egui-icons.ttf` | egui-icons, five-glyph subset of emoji-icon-font | John Slegers | [egui 6d518fc7](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts); included by default_fonts independently of monochrome_emoji_fonts | MIT, Copyright (c) 2014 John Slegers; full upstream text in `NOTICE` |
| (dependency) `epaint_default_fonts/fonts/Ubuntu-Light.ttf` | Ubuntu Light, default font fallback | Canonical Ltd. | [Pinned egui source](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts) | Ubuntu Font Licence 1.0; full upstream text and copyright in `NOTICE` |
| (dependency) `epaint_default_fonts/fonts/Hack-Regular.ttf` | Hack Regular, default monospace fallback | Source Foundry Authors; Bitstream Inc.; DejaVu contributors | [Pinned egui source](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts) | MIT and Bitstream Vera License; DejaVu work public domain; full upstream notices in `NOTICE` |

### D.E.V. App Suite icons

The launcher shows each app with that app's own icon, copied unmodified from the app's repository.
Refresh them from the source, never redraw them.

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/app-icons/draftcraft.png` | DraftCraft app icon (drafting compass on blueprint blue), 128 px | WithShalom Studios, for DraftCraft | [`DraftCraft-DEV/DraftCraft`](https://github.com/DraftCraft-DEV/DraftCraft) `assets/app-icon/hicolor/128x128/apps/` | Trademark of DraftCraft Engineering Ventures (that repository's `docs/brand/LICENSE-brand.txt`) |
| `assets/app-icons/tracedraw.png` | TraceDraw app icon, 128 px | The TraceDraw contributors | [`tracedraw/tracedraw`](https://github.com/tracedraw/tracedraw) at `0fdff91` | MIT OR Apache-2.0 |
