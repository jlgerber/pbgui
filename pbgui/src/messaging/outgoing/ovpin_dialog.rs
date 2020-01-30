use super::*;

#[derive(Debug, PartialEq)]
pub enum OVpinDialog {
    GetSites,
    GetRoles,
    GetLevels(String),
}

impl<'a> ToOMsg<'a> for OVpinDialog {
    fn to_omsg(self) -> OMsg<'a> {
        OMsg::VpinDialog(self)
    }
}
