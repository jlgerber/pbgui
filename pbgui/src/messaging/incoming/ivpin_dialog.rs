use super::*;

pub enum IVpinDialog {
    Roles(Vec<String>),
    Sites(Vec<String>),
    Levels(LevelMap),
}

impl<'a> ToIMsg<'a> for IVpinDialog {
    fn to_imsg(self) -> IMsg<'a> {
        IMsg::VpinDialog(self)
    }
}
