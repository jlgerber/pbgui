use super::*;

pub enum IVpinDialog {
    Roles(Vec<String>),
    Sites(Vec<String>),
    Levels(LevelMap),
}

impl ToIMsg for IVpinDialog {
    fn to_imsg(self) -> IMsg {
        IMsg::VpinDialog(self)
    }
}
