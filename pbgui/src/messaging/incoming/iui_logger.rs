use super::*;
use log::Level;

pub enum IUiLogger {
    Log {
        level: Level,
        target: String,
        file: Option<String>,
        line: Option<u32>,
        msg: String,
    },
}

impl ToIMsg for IUiLogger {
    fn to_imsg(self) -> IMsg {
        IMsg::UiLogger(self)
    }
}
