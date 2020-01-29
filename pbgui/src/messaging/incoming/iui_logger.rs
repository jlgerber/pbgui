use super::*;
use log::Level;

pub enum IUiLogger {
    Log(Option<Level>, String),
}

impl ToIMsg for IUiLogger {
    fn to_imsg(self) -> IMsg {
        IMsg::UiLogger(self)
    }
}
