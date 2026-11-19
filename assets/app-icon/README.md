# D.E.V. Launcher app icon

The app icon is the official **D.E.V. avatar**, exactly as supplied in
[`assets/brand/dev-avatar.svg`](../brand/dev-avatar.svg). Every file in this directory is rendered
from it (and the tray icons from [`assets/brand/dev-mark-mono.svg`](../brand/dev-mark-mono.svg)) by
`packaging/icons.sh`.

**These files are a trademark of DraftCraft Engineering Ventures. They are not open source and not
covered by D.E.V. Launcher's MIT or Apache-2.0 license.** Their terms are in
[`assets/brand/LICENSE-brand.txt`](../brand/LICENSE-brand.txt).

## Files

- `dev-launcher.svg`: a copy of `assets/brand/dev-avatar.svg`; also the hicolor scalable icon.
- `dev-launcher-1024.png`: 1024 px render, with transparent margin for Apple's icon grid.
- `dev-launcher.icns`: macOS bundle icon (`CFBundleIconFile`).
- `dev-launcher.ico`: Windows icon, 16–256 px, embedded in the `.exe` by
  `apps/dev-launcher/build.rs` and used by the MSI.
- `hicolor/<size>/apps/dev.draftcraft.Launcher.png` (16–512) and
  `hicolor/scalable/apps/dev.draftcraft.Launcher.svg`: Linux icon theme.
- `tray-mono-44.png` (macOS template, 22 pt @2x) and `tray-64.png` (Windows, Linux, FreeBSD): the
  tray / menu-bar icon.

## Regenerate

Run `packaging/icons.sh` on any OS. It needs `resvg`; the `.ico` and `.icns` are packed by
`cargo xtask ico`. Never edit these files by hand.
