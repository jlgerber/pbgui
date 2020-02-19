use super::*;
use log::Level;

#[derive(Debug, PartialEq)]
/// Requests originating in the main gui thread for the
/// ui logger element.
pub enum OUiLogger {
    /// Sends a particular log message
    SendLog {
        /// with the provided log level
        level: Level,
        /// given a supplied target
        target: String,
        /// file
        file: Option<String>,
        /// line number
        line: Option<u32>,
        /// and most importantly, the message
        msg: String,
    },
}

impl ToOMsg for OUiLogger {
    fn to_omsg(self) -> OMsg {
        OMsg::UiLogger(self)
    }
}
