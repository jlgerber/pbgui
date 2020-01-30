use super::*;

pub enum IMainToolbar {
    Shows(Vec<String>),
    Roles(Vec<String>),
    Platforms(Vec<String>),
    Sites(Vec<String>),
}

impl<'a> ToIMsg<'a> for IMainToolbar {
    fn to_imsg(self) -> IMsg<'a> {
        IMsg::MainToolbar(self)
    }
}
