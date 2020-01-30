use super::*;
use log::Level;

#[derive(Debug, PartialEq)]
pub enum OUiLogger {
    SendLog {
        level: Level,
        target: String,
        file: Option<String>,
        line: Option<u32>,
        msg: String,
    },
}

impl ToOMsg for OUiLogger {
    fn to_omsg(self) -> OMsg {
        OMsg::UiLogger(self)
    }
}
