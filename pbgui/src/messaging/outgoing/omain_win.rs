use super::*;

#[derive(Debug, PartialEq)]
pub enum OMainWin {
    GetVpins {
        level: String,
        role: String,
        platform: String,
        site: String,
        dir: String,
        package: Option<String>,
    },
}

impl ToOMsg for OMainWin {
    fn to_omsg(self) -> OMsg {
        OMsg::MainWin(self)
    }
}