use super::*;
use log::Level;

/// Responses returning to the main ui thread from the secondary thread
/// for the ui logger element.
pub enum IUiLogger {
    /// Returning log message
    Log {
        /// At the provided log level
        level: Level,
        /// with the supplied target name
        target: String,
        /// and optional file name
        file: Option<String>,
        /// and optional line number
        line: Option<u32>,
        /// with the supplied log message
        msg: String,
    },
}

impl ToIMsg for IUiLogger {
    fn to_imsg(self) -> IMsg {
        IMsg::UiLogger(self)
    }
}
