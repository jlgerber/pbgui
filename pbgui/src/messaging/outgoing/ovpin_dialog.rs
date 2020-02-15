use super::*;

#[derive(Debug, PartialEq)]
pub enum OVpinDialog {
    GetSites,
    GetRoles,
    GetLevels(String),
    SetVpin {
        dist: String,
        roles: Vec<String>,
        level: String,
        site: String,
        platform: String,
    },
}

impl ToOMsg for OVpinDialog {
    fn to_omsg(self) -> OMsg {
        OMsg::VpinDialog(self)
    }
}
