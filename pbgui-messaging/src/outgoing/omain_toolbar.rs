use super::*;

#[derive(Debug, PartialEq)]
pub enum OMainToolbar {
    GetShows,
    GetRoles,
    GetPlatforms,
    GetSites,
}

impl ToOMsg for OMainToolbar {
    fn to_omsg(self) -> OMsg {
        OMsg::MainToolbar(self)
    }
}
