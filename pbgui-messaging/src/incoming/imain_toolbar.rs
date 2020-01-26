use super::*;

pub enum IMainToolbar {
    Shows(Vec<String>),
    Roles(Vec<String>),
    Platforms(Vec<String>),
    Sites(Vec<String>),
}

impl ToIMsg for IMainToolbar {
    fn to_imsg(self) -> IMsg {
        IMsg::MainToolbar(self)
    }
}
