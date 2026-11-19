#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use serde::Deserialize;
use std::{env, error::Error, fmt::Write as _, fs, path::PathBuf};

#[derive(Deserialize)]
struct Manifest {
    apps: Vec<App>,
}
#[derive(Deserialize)]
struct App {
    id: String,
    name: String,
    tagline: String,
    color: [u8; 3],
    ink_light: [u8; 3],
    ink_dark: [u8; 3],
    repo: String,
    former_ids: Vec<String>,
    former_names: Vec<String>,
    kind: String,
    publisher: String,
    msi_publisher: Option<String>,
    bundle: Option<String>,
    former_bundles: Vec<String>,
    site: Option<String>,
    mac_team: Option<String>,
    mac_bundle_name: Option<String>,
    details: Option<Details>,
}
#[derive(Deserialize)]
struct Details {
    headline: String,
    about: String,
    platforms: String,
    #[serde(default)]
    features: Vec<Feature>,
    #[serde(default)]
    shots: Vec<Shot>,
}
#[derive(Deserialize)]
struct Feature {
    title: String,
    text: String,
}
#[derive(Deserialize)]
struct Shot {
    key: String,
    caption: String,
}
fn optional(s: &Option<String>) -> String {
    s.as_ref().map_or_else(|| "None".to_owned(), |v| format!("Some({v:?})"))
}
fn slice(items: &[String]) -> String {
    format!("&{items:?}")
}
/// The Rust source of one catalog: `pub const <suite>: [CraftApp; N]` and `pub const <details>: [AppDetails; M]`.
fn generate(path: &str, suite: &str, details_name: &str) -> Result<(String, String), Box<dyn Error>> {
    let manifest: Manifest = toml::from_str(&fs::read_to_string(path)?)?;
    let mut apps = format!("pub const {suite}: [CraftApp; {}] = [\n", manifest.apps.len());
    let count = manifest.apps.iter().filter(|a| a.details.is_some()).count();
    let mut details = format!("pub const {details_name}: [AppDetails; {count}] = [\n");
    for app in &manifest.apps {
        let kind = match app.kind.as_str() {
            "official" => "Official",
            "community" => "Community",
            _ => return Err(format!("unknown app kind for {}", app.id).into()),
        };
        writeln!(
            apps,
            "CraftApp {{ id: {:?}, name: {:?}, tagline: {:?}, color: {:?}, ink_light: {:?}, ink_dark: {:?}, repo: {:?}, former_ids: {}, former_names: {}, kind: AppKind::{kind}, publisher: {:?}, msi_publisher: {}, bundle: {}, former_bundles: {}, site: {}, mac_team: {}, mac_bundle_name: {} }},",
            app.id,
            app.name,
            app.tagline,
            app.color,
            app.ink_light,
            app.ink_dark,
            app.repo,
            slice(&app.former_ids),
            slice(&app.former_names),
            app.publisher,
            optional(&app.msi_publisher),
            optional(&app.bundle),
            slice(&app.former_bundles),
            optional(&app.site),
            optional(&app.mac_team),
            optional(&app.mac_bundle_name)
        )?;
        if let Some(d) = &app.details {
            write!(
                details,
                "AppDetails {{ id: {:?}, headline: {:?}, about: {:?}, platforms: {:?}, features: &[",
                app.id, d.headline, d.about, d.platforms
            )?;
            for f in &d.features {
                write!(details, "feature({:?}, {:?}),", f.title, f.text)?;
            }
            write!(details, "], shots: &[")?;
            for s in &d.shots {
                write!(details, "shot({:?}, {:?}),", s.key, s.caption)?;
            }
            writeln!(details, "] }},")?;
        }
    }
    apps.push_str("];\n");
    details.push_str("];\n");
    Ok((apps, details))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=catalog.toml");
    println!("cargo:rerun-if-changed=tests/fixtures/test-catalog.toml");
    let (mut apps, mut details) = generate("catalog.toml", "SUITE", "DETAILS")?;
    // Test builds of the other crates turn on `test-catalog`: the upstream app list, so their
    // tests can exercise former names and other publishers. Releases never do, and get none.
    if env::var_os("CARGO_FEATURE_TEST_CATALOG").is_some() {
        let (test_apps, test_details) = generate("tests/fixtures/test-catalog.toml", "TEST_SUITE", "TEST_DETAILS")?;
        apps.push_str(&test_apps);
        details.push_str(&test_details);
    } else {
        apps.push_str("pub const TEST_SUITE: [CraftApp; 0] = [];\n");
        details.push_str("pub const TEST_DETAILS: [AppDetails; 0] = [];\n");
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").ok_or("OUT_DIR missing")?);
    fs::write(out.join("catalog.rs"), apps)?;
    fs::write(out.join("details.rs"), details)?;
    Ok(())
}
