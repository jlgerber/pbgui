use super::*;
use log::Level;

#[derive(Debug, PartialEq)]
pub enum OUiLogger {
    SendLog(Option<Level>, String),
}

impl ToOMsg for OUiLogger {
    fn to_omsg(self) -> OMsg {
        OMsg::UiLogger(self)
    }
}
