use super::*;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;
pub enum IMainWin {
    Vpins(Vec<FindAllVersionPinsRow>),
}

impl ToIMsg for IMainWin {
    fn to_imsg(self) -> IMsg {
        IMsg::MainWin(self)
    }
}
