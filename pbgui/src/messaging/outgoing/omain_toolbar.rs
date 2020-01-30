use super::*;

#[derive(Debug, PartialEq)]
pub enum OMainToolbar {
    GetShows,
    GetRoles,
    GetPlatforms,
    GetSites,
}

impl<'a> ToOMsg<'a> for OMainToolbar {
    fn to_omsg(self) -> OMsg<'a> {
        OMsg::MainToolbar(self)
    }
}
