//! The apps D.E.V. Launcher installs: DraftCraft and the approved community apps.
//!
//! The list is data (`crates/core/catalog.toml`); [`SUITE`] is its built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// Who makes an app. Official apps live under [`GITHUB_OWNER`]; a community app is maintained
/// outside DraftCraft Engineering Ventures and is labelled as such wherever it is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppKind {
    Official,
    Community,
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// Display name (`PhotoCraft`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: getartcraft.com's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (`PdfCraft` shipped as `PrintCraft` until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (`PdfCraft`'s MSI said `PrintCraft` until 0.2.1).
    pub former_names: &'static [&'static str],
    /// The `.app` bundle stem on macOS when it differs from `name` (`ArtCraft` for `ArtCraft Studio`).
    pub mac_bundle_name: Option<&'static str>,
    pub kind: AppKind,
    /// Who publishes it, as the card's byline shows it (`ArtCraft`, `Tangency Labs`).
    pub publisher: &'static str,
    /// The publisher its Windows installer names, when it isn't ArtCraft's (`Learning Machines LLC`).
    pub msi_publisher: Option<&'static str>,
    /// The bundle id when it isn't `ai.storyteller.<id>` (a community app brings its own).
    pub bundle: Option<&'static str>,
    /// Older bundle ids its Flatpak shipped under (`SolidCraft` was `ai.storyteller.solidcraft`).
    pub former_bundles: &'static [&'static str],
    /// Its website, when it isn't a page on getartcraft.com (a community app has its own).
    pub site: Option<&'static str>,
    /// The Apple Developer team whose Developer ID signature its macOS build must carry. `None`
    /// while those builds carry no signature the launcher can check: it then opens a copy that's
    /// there and links to the releases, but installs nothing.
    pub mac_team: Option<&'static str>,
}

impl CraftApp {
    /// macOS bundle id, Flatpak id and Linux desktop-file id: the app's own, or `dev.draftcraft.<id>`.
    pub fn bundle_id(&self) -> String {
        self.bundle.map_or_else(|| format!("dev.draftcraft.{}", self.id), str::to_string)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's website: its own site, or its repository.
    pub fn website(&self) -> String {
        self.site.map_or_else(|| self.repo_url(), str::to_string)
    }

    /// Whether the launcher installs this app from a `kind` package.
    pub fn installs(&self, kind: crate::PackageKind) -> bool {
        kind != crate::PackageKind::Dmg || self.mac_team.is_some()
    }

    /// Whether this app runs on the given operating system.
    pub fn runs_on(&self, _os: crate::Os) -> bool {
        true
    }

    /// Bundle ids this app may be installed under, current first.
    pub fn bundle_ids(&self) -> impl Iterator<Item = String> + '_ {
        std::iter::once(self.bundle_id()).chain(self.former_bundles.iter().map(ToString::to_string))
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    /// The primary `.app` bundle stem on macOS (`ArtCraft` or `PhotoCraft`).
    pub fn mac_bundle_name(&self) -> &'static str {
        self.mac_bundle_name.unwrap_or(self.name)
    }

    /// Bundle stems a macOS `.app` may carry (`ArtCraft` and `ArtCraft Studio`), release name first.
    pub fn mac_bundle_names(&self) -> impl Iterator<Item = &'static str> + '_ {
        let primary = self.mac_bundle_name();
        std::iter::once(primary)
            .chain(std::iter::once(self.name).filter(move |&n| n != primary))
            .chain(self.former_names.iter().copied().filter(move |&n| n != primary && n != self.name))
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        crate::hosted_catalog::apps().iter().chain(SUITE.iter()).find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp {
        id,
        name,
        tagline,
        color,
        ink_light,
        ink_dark,
        repo,
        former_ids: &[],
        former_names: &[],
        mac_bundle_name: None,
        kind: AppKind::Official,
        publisher: "DraftCraft Engineering Ventures",
        msi_publisher: None,
        bundle: None,
        former_bundles: &[],
        site: None,
        mac_team: None,
    }
}

// The bundled catalog, in display order: DraftCraft, then the approved community apps.
include!(concat!(env!("OUT_DIR"), "/catalog.rs"));

/// The organization's website.
pub const WEBSITE_URL: &str = "https://draftcraft.dev";
/// The GitHub account every official app's repository lives under. The release feed can only
/// move an app within it; a community app installs from its own repo, pinned in [`SUITE`].
pub const GITHUB_OWNER: &str = "DraftCraft-DEV";
/// DraftCraft Engineering Ventures on GitHub.
pub const GITHUB_URL: &str = "https://github.com/DraftCraft-DEV";
/// Social accounts: (service, URL), shown as small icons. None yet.
pub const SOCIAL: [(&str, &str); 0] = [];

/// The DraftCraft community on Discord.
pub const DISCORD_URL: &str = "https://discord.gg/YgRK8Eww9x";
/// Deep link into the Discord application for the same invite.
pub const DISCORD_APP_URL: &str = "discord://-/invite/YgRK8Eww9x";

/// D.E.V. Launcher itself, described like an app so it can update itself with the same machinery
/// (asset names `dev-launcher-<ver>-…`, bundle id `dev.draftcraft.Launcher`, `D.E.V. Launcher.app`).
/// Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = CraftApp {
    bundle: Some("dev.draftcraft.Launcher"),
    ..app(
        "dev-launcher",
        "D.E.V. Launcher",
        "Installs and updates DraftCraft and its community apps",
        [[0x1f, 0x5f, 0xaf], [0x18, 0x4a, 0x8c], [0x8d, 0xb8, 0xf0]],
        "DraftCraft-DEV/launcher",
    )
};

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "DraftCraft-DEV/launcher";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "dev-launcher";

/// An app's 128 px icon (PNG), as copied from its repo into `assets/app-icons`: the UI draws it,
/// and the Linux install puts it in the desktop menu.
pub fn app_icon_png(id: &str) -> Option<&'static [u8]> {
    // Test builds of the other crates: every fixture app borrows a real icon.
    #[cfg(feature = "test-catalog")]
    if TEST_SUITE.iter().any(|a| a.id == id) {
        return Some(include_bytes!("../../../assets/app-icons/draftcraft.png"));
    }
    Some(match id {
        "draftcraft" => include_bytes!("../../../assets/app-icons/draftcraft.png"),
        "tracedraw" => include_bytes!("../../../assets/app-icons/tracedraw.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_app_has_its_icon() {
        for app in &SUITE {
            let png = app_icon_png(app.id).unwrap_or_else(|| panic!("no icon for {}", app.id));
            assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "{} icon isn't a PNG", app.id);
        }
        assert!(app_icon_png("notacraft").is_none());
    }

    /// The catalog is DraftCraft and TraceDraw, each pinned to its own repository.
    #[test]
    fn catalog_is_draftcraft_and_tracedraw() {
        let ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        assert_eq!(ids, ["draftcraft", "tracedraw"]);
        for a in &SUITE {
            assert_eq!(a.name.to_lowercase(), a.id);
            match a.kind {
                AppKind::Official => assert!(a.repo.starts_with(&format!("{GITHUB_OWNER}/")), "{} is official", a.id),
                AppKind::Community => assert!(!a.repo.starts_with(&format!("{GITHUB_OWNER}/")), "{} is community", a.id),
            }
        }
    }

    #[test]
    fn draftcraft_installs_from_its_own_repository() {
        let dc = CraftApp::by_id("draftcraft").unwrap();
        assert_eq!((dc.kind, dc.repo), (AppKind::Official, "DraftCraft-DEV/DraftCraft"));
        assert_eq!(dc.bundle_id(), "dev.skryx.draftcraft");
        assert_eq!(dc.msi_publisher, Some("Skryx Dev"));
        assert_eq!(dc.website(), "https://github.com/DraftCraft-DEV/DraftCraft");
    }

    #[test]
    fn tracedraw_is_a_community_app_from_its_official_releases() {
        let td = CraftApp::by_id("tracedraw").unwrap();
        assert_eq!((td.kind, td.repo), (AppKind::Community, "tracedraw/tracedraw"));
        assert_eq!(td.bundle_id(), "dev.tracedraw.app");
        assert_eq!(td.msi_publisher, Some("TraceDraw"));
    }

    /// No Developer ID team is known for either app's macOS builds yet, so neither DMG is
    /// installed: a copy that's there is opened, and the card links to the releases.
    #[test]
    fn no_mac_install_without_a_known_signing_team() {
        for a in &SUITE {
            assert_eq!(a.mac_team, None, "{}", a.id);
            assert!(!a.installs(crate::PackageKind::Dmg), "{}", a.id);
            assert!(a.installs(crate::PackageKind::PortableZip) && a.installs(crate::PackageKind::AppImage), "{}", a.id);
        }
    }

    #[test]
    fn the_launcher_is_dev_launcher() {
        assert_eq!((LAUNCHER.id, LAUNCHER.name, LAUNCHER.repo), ("dev-launcher", "D.E.V. Launcher", "DraftCraft-DEV/launcher"));
        assert_eq!(LAUNCHER.bundle_id(), "dev.draftcraft.Launcher");
        assert_eq!(LAUNCHER_REPO, LAUNCHER.repo);
    }
}
