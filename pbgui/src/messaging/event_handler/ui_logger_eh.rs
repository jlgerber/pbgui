//! Provides a function that processes `messaging::event::UiLogger` events, updating the ui state or
//! logging errors
use super::*;
use crate::messaging::{event::ui_logger::UiLogger, incoming::IUiLogger};
use pbgui_logger::{inner_log_win::LogData, log_win::LogWin};
use std::rc::Rc;
pub fn match_ui_logger<'a>(event: UiLogger, logger: Rc<LogWin>, receiver: &Receiver<IMsg>) {
    match event {
        UiLogger::SendLog => {
            if let Ok(IMsg::UiLogger(IUiLogger::Log {
                level,
                target,
                file,
                line,
                msg,
            })) = receiver.recv()
            {
                let log_data = LogData {
                    //level,
                    target: target.as_str(),
                    file: file.as_deref(),
                    line,
                };
                unsafe {
                    let mut log_data = Some(log_data);
                    for log in msg.split("\n") {
                        logger.log(level, log_data, log);
                        log_data = None
                    }
                }
            } else {
                log::error!("UiLogger::SendLog IMsg does not match event state");
            }
        }
    }
}
