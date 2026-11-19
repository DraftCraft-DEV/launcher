//! The catalog the launcher runs with: the bundled one ([`SUITE`], from `catalog.toml`).
//!
//! There is no hosted catalog to download. Every app, its details page and its icon come from the
//! build, so nothing here reaches the network. Callers read the catalog through this module, so a
//! hosted, signed catalog could be added back behind the same functions.

use crate::catalog::CraftApp;
use crate::details::AppDetails;

/// Desktop metadata a hosted catalog could add. The bundled catalog has none.
pub struct Metadata {
    pub desktop_categories: String,
    pub startup_wm_class: String,
    pub icon_sha256: String,
}

/// The apps, in display order. Test builds of the other crates (feature `test-catalog`) run on
/// the upstream test fixture instead, so their tests keep exercising the same machinery.
pub fn apps() -> &'static [CraftApp] {
    #[cfg(feature = "test-catalog")]
    return &crate::catalog::TEST_SUITE;
    #[cfg(not(feature = "test-catalog"))]
    &crate::catalog::SUITE
}

/// A details page from a hosted catalog: none, so the bundled page is used.
pub fn details(_id: &str) -> Option<&'static AppDetails> {
    None
}

/// Desktop metadata from a hosted catalog: none, so the bundled values are used.
pub fn metadata(_id: &str) -> Option<&'static Metadata> {
    None
}

/// An icon from a hosted catalog: none, so the bundled icon is used.
pub fn icon(_id: &str) -> Option<&'static [u8]> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::SUITE;

    #[cfg(not(feature = "test-catalog"))]
    #[test]
    fn the_catalog_is_the_bundled_one() {
        assert_eq!(apps().iter().map(|a| a.id).collect::<Vec<_>>(), SUITE.iter().map(|a| a.id).collect::<Vec<_>>());
        assert!(SUITE.iter().all(|a| details(a.id).is_none() && metadata(a.id).is_none() && icon(a.id).is_none()));
    }
}
