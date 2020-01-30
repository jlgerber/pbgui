use super::*;
use crate::messaging::{event::ui_logger::UiLogger, incoming::IUiLogger};
use pbgui_logger::log_win::LogWin;
use std::rc::Rc;
pub fn match_ui_logger<'a>(event: UiLogger, logger: Rc<LogWin>, receiver: &Receiver<IMsg>) {
    match event {
        UiLogger::SendLog => {
            if let Ok(IMsg::UiLogger(IUiLogger::Log(record))) = receiver.recv() {
                unsafe {
                    let msg = format!("{}", record.args());
                    let mut level = Some(record.level());
                    for log in msg.split("\n") {
                        logger.log(level, log);
                        level = None
                    }
                }
            } else {
                log::error!("UiLogger::SendLog IMsg does not match event state");
            }
        }
    }
}
