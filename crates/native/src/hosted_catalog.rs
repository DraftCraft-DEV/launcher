//! Loading a hosted catalog: there is none. The launcher runs on the bundled catalog
//! (`artcraft_launcher_core::hosted_catalog`), so loading asks nothing of the network.

use crate::paths::Paths;

/// Kept so the backend's startup order stays as it is; nothing is fetched or cached.
pub fn load_once(_http: Option<&crate::http::Http>, _paths: &Paths) {}
