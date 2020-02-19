use super::*;

#[derive(Debug, PartialEq)]
/// Requests originating from the main ui thread for the
/// version pin dialog.
pub enum OVpinDialog {
    /// Retrieve the list of sites
    GetSites,
    /// Retrieve the list of roles
    GetRoles,
    /// Retrieve the list of roles for the provided show
    GetLevels(String),
    /// Add a versionpin or versionpins to the show,
    SetVpin {
        /// for the provided  distribution
        dist: String,
        /// and one or more roles
        roles: Vec<String>,
        /// at the supplied level
        level: String,
        /// and site
        site: String,
        /// and platform
        platform: String,
    },
}

impl ToOMsg for OVpinDialog {
    fn to_omsg(self) -> OMsg {
        OMsg::VpinDialog(self)
    }
}
