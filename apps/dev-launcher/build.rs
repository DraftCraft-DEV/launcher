//! Windows only: embed the app icon and version info (VERSIONINFO) into `dev-launcher.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `DEV_LAUNCHER_REQUIRE_WINRES=1` (set
//! by the release packaging) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/dev-launcher.ico");
    println!("cargo:rerun-if-env-changed=DEV_LAUNCHER_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/dev-launcher.ico")
        .set("ProductName", "D.E.V. Launcher")
        .set("FileDescription", "D.E.V. Launcher")
        .set("CompanyName", "DraftCraft Engineering Ventures")
        .set("LegalCopyright", "Copyright (c) the D.E.V. Launcher authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "dev-launcher.exe")
        .set("InternalName", "dev-launcher");
    if let Err(e) = res.compile() {
        if std::env::var_os("DEV_LAUNCHER_REQUIRE_WINRES").is_some() {
            println!("cargo:warning=embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=dev-launcher.exe built without icon/version resources: {e}");
    }
}
