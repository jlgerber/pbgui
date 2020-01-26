use super::*;

#[derive(Debug, PartialEq)]
pub enum OVpinDialog {
    GetSites,
    GetRoles,
    GetLevels(String),
}

impl ToOMsg for OVpinDialog {
    fn to_omsg(self) -> OMsg {
        OMsg::VpinDialog(self)
    }
}
