use super::*;
use crate::messaging::{event::ui_logger::UiLogger, incoming::IUiLogger};
use pbgui_logger::log_win::LogWin;
use std::rc::Rc;
pub fn match_ui_logger<'a>(event: UiLogger, logger: Rc<LogWin>, receiver: &Receiver<IMsg>) {
    match event {
        UiLogger::SendLog => {
            if let Ok(IMsg::UiLogger(IUiLogger::Log {
                level,
                target,
                module_path,
                file,
                line,
                msg,
            })) = receiver.recv()
            {
                unsafe {
                    let mut level = Some(level);
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
