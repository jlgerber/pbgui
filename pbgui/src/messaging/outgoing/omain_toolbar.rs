use super::*;

#[derive(Debug, PartialEq)]
/// Requests outgoing from the main gui thread for the
/// main toolbar
pub enum OMainToolbar {
    /// Request the known shows.
    GetShows,
    /// Request the known  roles
    GetRoles,
    /// Request the known platforms
    GetPlatforms,
    /// Request the known sites
    GetSites,
}

impl ToOMsg for OMainToolbar {
    fn to_omsg(self) -> OMsg {
        OMsg::MainToolbar(self)
    }
}
