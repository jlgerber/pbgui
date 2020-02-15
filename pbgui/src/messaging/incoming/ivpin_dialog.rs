use super::*;

pub enum IVpinDialog {
    Roles(Vec<String>),
    Sites(Vec<String>),
    Levels(LevelMap),
    // this should probably be a SetVpinOk(Vec<AddVpinRow>) or a SetVpinFailed{err: String}
    SetVpin(bool),
}

impl ToIMsg for IVpinDialog {
    fn to_imsg(self) -> IMsg {
        IMsg::VpinDialog(self)
    }
}
