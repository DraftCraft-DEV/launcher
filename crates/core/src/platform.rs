//! Platforms, package formats, and choosing the right release asset.
//!
//! Release files are named `<app>-<version>-<os>-<arch>[-portable].<ext>`, for example
//! `tracedraw-0.48.0-windows-x64-portable.zip` or `tracedraw-0.48.0-linux-arm64.AppImage`. DraftCraft
//! names them without the version (`draftcraft-windows-x64.zip`, `draftcraft-linux-aarch64.AppImage`):
//! the release's tag carries it.
//!
//! Labels are English. The UI translates [`PackageKind::label`]; OS and architecture names are
//! shown as they are.

use serde::{Deserialize, Serialize};

use crate::release::{Asset, Release};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Os {
    Windows,
    MacOs,
    Linux,
    FreeBsd,
}

impl Os {
    pub fn label(self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::MacOs => "macOS",
            Self::Linux => "Linux",
            Self::FreeBsd => "FreeBSD",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Arch {
    X64,
    X86,
    Arm64,
    Riscv64,
}

impl Arch {
    pub fn label(self) -> &'static str {
        match self {
            Self::X64 => "x64",
            Self::X86 => "x86",
            Self::Arm64 => "ARM64",
            Self::Riscv64 => "RISC-V",
        }
    }
}

/// A package format a release ships.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackageKind {
    /// Windows Installer package (per user by default; an existing per-machine copy stays per-machine).
    Msi,
    /// Windows zip with the exe at the top level.
    PortableZip,
    /// macOS disk image with the `.app` bundle.
    Dmg,
    AppImage,
    Deb,
    Rpm,
    /// Arch Linux package (`.pkg.tar.zst`).
    ArchPkg,
    Flatpak,
    /// Linux or FreeBSD tarball (`bin/<app>`, `share/…`).
    TarGz,
}

impl PackageKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Msi => "Installer (.msi)",
            Self::PortableZip => "Portable (.zip)",
            Self::Dmg => "Disk image (.dmg)",
            Self::AppImage => "AppImage",
            Self::Deb => "Debian package (.deb)",
            Self::Rpm => "RPM package (.rpm)",
            Self::ArchPkg => "Arch package (.pkg.tar.zst)",
            Self::Flatpak => "Flatpak bundle",
            Self::TarGz => "Tarball (.tar.gz)",
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::Msi => ".msi",
            Self::PortableZip => "-portable.zip",
            Self::Dmg => ".dmg",
            Self::AppImage => ".AppImage",
            Self::Deb => ".deb",
            Self::Rpm => ".rpm",
            Self::ArchPkg => ".pkg.tar.zst",
            Self::Flatpak => ".flatpak",
            Self::TarGz => ".tar.gz",
        }
    }
}

/// Where the launcher is running (or, on the web, the visitor's machine).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Platform {
    pub os: Os,
    pub arch: Arch,
}

impl Platform {
    /// The platform this binary was built for, or `None` for wasm and unsupported targets.
    pub fn current() -> Option<Self> {
        let os = if cfg!(target_os = "windows") {
            Os::Windows
        } else if cfg!(target_os = "macos") {
            Os::MacOs
        } else if cfg!(target_os = "linux") {
            Os::Linux
        } else if cfg!(target_os = "freebsd") {
            Os::FreeBsd
        } else {
            return None;
        };
        let arch = if cfg!(target_arch = "x86_64") {
            Arch::X64
        } else if cfg!(target_arch = "x86") {
            Arch::X86
        } else if cfg!(target_arch = "aarch64") {
            Arch::Arm64
        } else if cfg!(target_arch = "riscv64") {
            Arch::Riscv64
        } else {
            return None;
        };
        Some(Self { os, arch })
    }

    pub fn label(self) -> String {
        match self.os {
            Os::MacOs => "macOS".into(),
            os => format!("{} {}", os.label(), self.arch.label()),
        }
    }

    /// Asset platform tokens to try, best first. Windows on ARM and 64-bit Windows can fall back
    /// to emulated builds; macOS prefers universal, then native builds. Apple silicon can fall
    /// back to Intel builds, but Intel Macs cannot run ARM builds.
    fn tokens(self) -> &'static [&'static str] {
        match (self.os, self.arch) {
            (Os::Windows, Arch::X64) => &["windows-x64", "windows-x86"],
            (Os::Windows, Arch::X86) => &["windows-x86"],
            (Os::Windows, Arch::Arm64) => &["windows-arm64", "windows-x64", "windows-x86"],
            (Os::MacOs, Arch::X64) => &["macos-universal", "macos-x86_64"],
            (Os::MacOs, Arch::Arm64) => &["macos-universal", "macos-aarch64", "macos-arm64", "macos-x86_64"],
            (Os::Linux, Arch::X64) => &["linux-x86_64"],
            (Os::Linux, Arch::Arm64) => &["linux-aarch64", "linux-arm64"],
            (Os::Linux, Arch::Riscv64) => &["linux-riscv64"],
            (Os::FreeBsd, Arch::X64) => &["freebsd-x86_64"],
            (Os::FreeBsd, Arch::Arm64) => &["freebsd-aarch64"],
            (Os::Windows | Os::MacOs | Os::FreeBsd, _) | (Os::Linux, Arch::X86) => &[],
        }
    }

    /// Package formats the launcher installs on this OS, in order of preference.
    pub fn default_kinds(self) -> &'static [PackageKind] {
        match self.os {
            Os::Windows => &[PackageKind::Msi, PackageKind::PortableZip],
            Os::MacOs => &[PackageKind::Dmg],
            Os::Linux => &[PackageKind::AppImage, PackageKind::TarGz],
            Os::FreeBsd => &[PackageKind::TarGz],
        }
    }

    /// Every format offered as a manual download on this OS (the web build's download menu).
    pub fn download_kinds(self) -> &'static [PackageKind] {
        match self.os {
            Os::Windows => &[PackageKind::Msi, PackageKind::PortableZip],
            Os::MacOs => &[PackageKind::Dmg],
            Os::Linux => &[PackageKind::AppImage, PackageKind::Deb, PackageKind::Rpm, PackageKind::ArchPkg, PackageKind::Flatpak, PackageKind::TarGz],
            Os::FreeBsd => &[PackageKind::TarGz],
        }
    }
}

/// What an asset name says about itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetName {
    pub stem: String,
    /// The version the name carries, or `""` for a name without one (DraftCraft's).
    pub version: String,
    pub token: String,
    pub kind: PackageKind,
}

impl AssetName {
    /// The architecture the asset is built for: the last part of its token (`windows-x86` → x86).
    /// `None` for a token that names none (`macos-universal`).
    pub fn arch(&self) -> Option<Arch> {
        match self.token.rsplit('-').next()? {
            "x64" | "x86_64" => Some(Arch::X64),
            "x86" => Some(Arch::X86),
            "arm64" | "aarch64" => Some(Arch::Arm64),
            "riscv64" => Some(Arch::Riscv64),
            _ => None,
        }
    }
}

/// Strips `<stem><sep>` (ASCII case-insensitively) from the start of `base`, returning the rest.
fn strip_stem_prefix<'a>(base: &'a str, stem: &str, sep: char) -> Option<&'a str> {
    if base.get(..stem.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(stem)) && base.as_bytes().get(stem.len()) == Some(&(sep as u8)) {
        Some(&base[stem.len() + 1..])
    } else {
        None
    }
}

/// The operating systems a release file name can name.
const OS_TOKENS: [&str; 4] = ["windows", "macos", "linux", "freebsd"];

/// `<os>-<arch>` with a known OS and an architecture without `-`, as a token.
fn os_arch_token(rest: &str) -> Option<String> {
    let (os, arch) = rest.split_once('-')?;
    (OS_TOKENS.contains(&os) && !arch.is_empty() && !arch.contains('-')).then(|| format!("{os}-{arch}"))
}

/// Reads `<stem>-<version>-<os>-<arch>[-portable].<ext>`, the versionless `<stem>-<os>-<arch>.<ext>`
/// (a Windows `.zip` is then the portable build), or Studio's `<Stem>_<version>_<arch>[_<locale>].<ext>`;
/// `None` for checksums, zsync files, CLI and web bundles, setup programs, or anything else.
pub fn parse_asset_name(name: &str, stems: &[&str]) -> Option<AssetName> {
    if let Some(parsed) = parse_versionless(name, stems) {
        return Some(parsed);
    }
    const KINDS: [PackageKind; 9] = [
        PackageKind::PortableZip,
        PackageKind::TarGz,
        PackageKind::Msi,
        PackageKind::Dmg,
        PackageKind::AppImage,
        PackageKind::Deb,
        PackageKind::Rpm,
        PackageKind::ArchPkg,
        PackageKind::Flatpak,
    ];
    let (kind, base) = KINDS.iter().find_map(|k| name.strip_suffix(k.suffix()).map(|b| (*k, b)))?;
    for stem in stems {
        if let Some(rest) = strip_stem_prefix(base, stem, '-') {
            // rest = <version>-<os>-<arch>; os and arch never contain '-', versions may.
            let mut parts = rest.rsplitn(3, '-');
            let (Some(arch), Some(os), Some(version)) = (parts.next(), parts.next(), parts.next()) else { continue };
            if crate::version::parse(version).is_none() || !version.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            return Some(AssetName { stem: (*stem).to_string(), version: version.to_string(), token: format!("{os}-{arch}"), kind });
        }
        if let Some(rest) = strip_stem_prefix(base, stem, '_') {
            // Studio / Tauri format: <version>_<arch>[_<locale>] where <arch> may be `x86_64`.
            let Some((version, suffix)) = rest.split_once('_') else { continue };
            if crate::version::parse(version).is_none() || !version.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            let arch_token = if suffix == "x86_64" || suffix.starts_with("x86_64_") { "x86_64" } else { suffix.split('_').next().unwrap_or("") };
            let token = match (kind, arch_token) {
                (PackageKind::Dmg, "universal") => Some("macos-universal".to_string()),
                (PackageKind::Dmg, "x86_64" | "x64") => Some("macos-x86_64".to_string()),
                (PackageKind::Dmg, "aarch64" | "arm64") => Some("macos-aarch64".to_string()),
                (PackageKind::Msi | PackageKind::PortableZip, "x86_64" | "x64") => Some("windows-x64".to_string()),
                (PackageKind::Msi | PackageKind::PortableZip, "x86") => Some("windows-x86".to_string()),
                (PackageKind::Msi | PackageKind::PortableZip, "arm64" | "aarch64") => Some("windows-arm64".to_string()),
                _ => None,
            };
            if let Some(token) = token {
                return Some(AssetName { stem: (*stem).to_string(), version: version.to_string(), token, kind });
            }
        }
    }
    None
}

/// `<stem>-<os>-<arch>.<ext>`, as DraftCraft names its files: no version (the release tag has it).
fn parse_versionless(name: &str, stems: &[&str]) -> Option<AssetName> {
    const KINDS: [PackageKind; 8] = [
        PackageKind::PortableZip,
        PackageKind::TarGz,
        PackageKind::Msi,
        PackageKind::Dmg,
        PackageKind::AppImage,
        PackageKind::Deb,
        PackageKind::Rpm,
        PackageKind::Flatpak,
    ];
    // A plain `.zip` names the portable build only on Windows (`draftcraft-windows-x64.zip`).
    let (kind, base) = KINDS
        .iter()
        .find_map(|k| name.strip_suffix(k.suffix()).map(|b| (*k, b)))
        .or_else(|| name.strip_suffix(".zip").filter(|b| b.contains("-windows-")).map(|b| (PackageKind::PortableZip, b)))?;
    stems.iter().find_map(|stem| {
        let token = os_arch_token(strip_stem_prefix(base, stem, '-')?)?;
        let windows_only = matches!(kind, PackageKind::Msi | PackageKind::PortableZip);
        if windows_only != token.starts_with("windows-") {
            return None;
        }
        Some(AssetName { stem: (*stem).to_string(), version: String::new(), token, kind })
    })
}

/// The best asset of `release` for `platform`, trying `kinds` in order and, within a kind, the
/// platform's tokens in order (native before emulated).
pub fn pick_asset<'a>(release: &'a Release, stems: &[&str], platform: Platform, kinds: &[PackageKind]) -> Option<(&'a Asset, PackageKind)> {
    let parsed: Vec<(&Asset, AssetName)> = release.assets.iter().filter_map(|a| parse_asset_name(&a.name, stems).map(|n| (a, n))).collect();
    for kind in kinds {
        for token in platform.tokens() {
            if let Some((asset, _)) = parsed.iter().find(|(_, n)| n.kind == *kind && n.token == *token) {
                return Some((asset, *kind));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(names: &[&str]) -> Release {
        Release {
            assets: names.iter().map(|n| Asset { name: (*n).into(), size: 1, url: format!("https://x/{n}"), sha256: None }).collect(),
            ..Release::default()
        }
    }

    fn pick<'a>(r: &'a Release, stem: &str, os: Os, arch: Arch, kinds: &[PackageKind]) -> Option<&'a str> {
        pick_asset(r, &[stem], Platform { os, arch }, kinds).map(|(a, _)| a.name.as_str())
    }

    fn draftcraft() -> Release {
        crate::release::parse_github_release(include_str!("../tests/fixtures/draftcraft-release.json"), "DraftCraft-DEV/DraftCraft").unwrap()
    }

    /// TraceDraw's file names, as its release workflow writes them (`.github/workflows/build.yml`
    /// and `packaging/` in tracedraw/tracedraw).
    const TRACEDRAW: &[&str] = &[
        "tracedraw-0.48.0-windows-x64-setup.exe",
        "tracedraw-0.48.0-windows-x64-portable.zip",
        "tracedraw-0.48.0-windows-arm64-setup.exe",
        "tracedraw-0.48.0-windows-arm64-portable.zip",
        "tracedraw-0.48.0-macos-arm64.dmg",
        "tracedraw-0.48.0-macos-x86_64.dmg",
        "tracedraw-0.48.0-linux-x86_64.AppImage",
        "tracedraw-0.48.0-linux-x86_64.AppImage.zsync",
        "tracedraw-0.48.0-linux-x86_64.tar.gz",
        "tracedraw-0.48.0-linux-arm64.AppImage",
        "tracedraw-0.48.0-linux-arm64.tar.gz",
        "tracedraw_0.48.0-1_amd64.deb",
        "tracedraw-0.48.0-web.zip",
    ];

    #[test]
    fn draftcraft_picks_its_published_files() {
        let r = draftcraft();
        assert_eq!(r.version.as_ref().map(ToString::to_string).as_deref(), Some("2.2.0"));
        let zip = [PackageKind::PortableZip];
        assert_eq!(pick(&r, "draftcraft", Os::Windows, Arch::X64, &zip), Some("draftcraft-windows-x64.zip"));
        assert_eq!(pick(&r, "draftcraft", Os::Windows, Arch::Arm64, &zip), Some("draftcraft-windows-arm64.zip"));
        let linux = [PackageKind::AppImage, PackageKind::TarGz];
        assert_eq!(pick(&r, "draftcraft", Os::Linux, Arch::X64, &linux), Some("draftcraft-linux-x86_64.AppImage"));
        assert_eq!(pick(&r, "draftcraft", Os::Linux, Arch::Arm64, &linux), Some("draftcraft-linux-aarch64.AppImage"));
        assert_eq!(pick(&r, "draftcraft", Os::Linux, Arch::X64, &[PackageKind::Deb]), Some("draftcraft-linux-x86_64.deb"));
        let dmg = [PackageKind::Dmg];
        assert_eq!(pick(&r, "draftcraft", Os::MacOs, Arch::Arm64, &dmg), Some("draftcraft-macos-arm64.dmg"));
        assert_eq!(pick(&r, "draftcraft", Os::MacOs, Arch::X64, &dmg), Some("draftcraft-macos-x86_64.dmg"));
    }

    /// DraftCraft's names carry no version: the release's tag does, and every file has a checksum.
    #[test]
    fn draftcraft_names_carry_no_version_and_every_file_a_checksum() {
        let r = draftcraft();
        let n = parse_asset_name("draftcraft-windows-x64.zip", &["draftcraft"]).unwrap();
        assert_eq!((n.version.as_str(), n.token.as_str(), n.kind), ("", "windows-x64", PackageKind::PortableZip));
        assert!(r.assets.iter().all(|a| a.sha256.is_some()), "a DraftCraft file has no checksum");
        for other in [
            "draftcraft-windows-x64-setup.exe",
            "draftcraft-cli-macos-arm64.zip",
            "draftcraft-cli-macos-x86_64.zip",
            "draftcraft-macos-arm64.zip",
            "draftcraft-linux-x86_64.zip",
            "SHA256SUMS.txt",
        ] {
            assert_eq!(parse_asset_name(other, &["draftcraft"]), None, "{other}");
        }
    }

    #[test]
    fn tracedraw_picks_its_published_files() {
        let r = release(TRACEDRAW);
        let zip = [PackageKind::PortableZip];
        assert_eq!(pick(&r, "tracedraw", Os::Windows, Arch::X64, &zip), Some("tracedraw-0.48.0-windows-x64-portable.zip"));
        assert_eq!(pick(&r, "tracedraw", Os::Windows, Arch::Arm64, &zip), Some("tracedraw-0.48.0-windows-arm64-portable.zip"));
        let linux = [PackageKind::AppImage, PackageKind::TarGz];
        assert_eq!(pick(&r, "tracedraw", Os::Linux, Arch::X64, &linux), Some("tracedraw-0.48.0-linux-x86_64.AppImage"));
        assert_eq!(pick(&r, "tracedraw", Os::Linux, Arch::Arm64, &linux), Some("tracedraw-0.48.0-linux-arm64.AppImage"));
        assert_eq!(pick(&r, "tracedraw", Os::Linux, Arch::Arm64, &[PackageKind::TarGz]), Some("tracedraw-0.48.0-linux-arm64.tar.gz"));
        assert_eq!(pick(&r, "tracedraw", Os::MacOs, Arch::Arm64, &[PackageKind::Dmg]), Some("tracedraw-0.48.0-macos-arm64.dmg"));
        let n = parse_asset_name("tracedraw-0.48.0-linux-arm64.AppImage", &["tracedraw"]).unwrap();
        assert_eq!((n.version.as_str(), n.arch()), ("0.48.0", Some(Arch::Arm64)));
        for other in ["tracedraw-0.48.0-windows-x64-setup.exe", "tracedraw-0.48.0-web.zip", "tracedraw_0.48.0-1_amd64.deb"] {
            assert_eq!(parse_asset_name(other, &["tracedraw"]), None, "{other}");
        }
    }

    #[test]
    fn windows_on_arm_falls_back_to_the_x64_build() {
        let r = release(&["app-1.0.0-windows-x64-portable.zip", "app-1.0.0-windows-x86-portable.zip"]);
        assert_eq!(pick(&r, "app", Os::Windows, Arch::Arm64, &[PackageKind::PortableZip]), Some("app-1.0.0-windows-x64-portable.zip"));
        assert_eq!(pick(&r, "app", Os::Windows, Arch::X86, &[PackageKind::PortableZip]), Some("app-1.0.0-windows-x86-portable.zip"));
    }

    #[test]
    fn intel_macs_never_pick_an_arm_build_and_apple_silicon_keeps_the_intel_fallback() {
        let arm_only = release(&["app-1.0.0-macos-arm64.dmg"]);
        assert_eq!(pick(&arm_only, "app", Os::MacOs, Arch::X64, &[PackageKind::Dmg]), None);
        let intel_only = release(&["app-1.0.0-macos-x86_64.dmg"]);
        assert_eq!(pick(&intel_only, "app", Os::MacOs, Arch::Arm64, &[PackageKind::Dmg]), Some("app-1.0.0-macos-x86_64.dmg"));
        let both = release(&["app-1.0.0-macos-x86_64.dmg", "app-1.0.0-macos-universal.dmg", "app-1.0.0-macos-arm64.dmg"]);
        assert_eq!(pick(&both, "app", Os::MacOs, Arch::Arm64, &[PackageKind::Dmg]), Some("app-1.0.0-macos-universal.dmg"));
    }

    #[test]
    fn linux_never_picks_another_processor() {
        let r = release(&["app-linux-x86_64.AppImage", "app-linux-x86_64.deb"]);
        for kind in [PackageKind::AppImage, PackageKind::Deb] {
            assert_eq!(pick(&r, "app", Os::Linux, Arch::Arm64, &[kind]), None);
            assert!(pick(&r, "app", Os::Linux, Arch::X64, &[kind]).is_some());
        }
    }

    #[test]
    fn versioned_names_keep_their_version_and_architecture() {
        let n = parse_asset_name("app-0.2.0-rc.1-windows-arm64-portable.zip", &["app"]).unwrap();
        assert_eq!((n.version.as_str(), n.token.as_str(), n.kind), ("0.2.0-rc.1", "windows-arm64", PackageKind::PortableZip));
        assert_eq!(parse_asset_name("app-1.0.0-macos-universal.dmg", &["app"]).unwrap().arch(), None);
        for other in ["app-cli-1.0.0-macos-universal.zip", "app-web-1.0.0.zip", "app-1.0.0-linux-x86_64.AppImage.zsync", "other-1.0.0-linux-x86_64.AppImage"] {
            assert_eq!(parse_asset_name(other, &["app"]), None, "{other}");
        }
    }
}
