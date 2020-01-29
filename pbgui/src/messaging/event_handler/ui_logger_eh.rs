use super::*;
use crate::messaging::{event::ui_logger::UiLogger, incoming::IUiLogger};
use pbgui_logger::log_win::LogWin;
use std::rc::Rc;

pub fn match_ui_logger<'a>(event: UiLogger, logger: Rc<LogWin>, receiver: &Receiver<IMsg>) {
    match event {
        UiLogger::SendLog => {
            if let Ok(IMsg::UiLogger(IUiLogger::Log(level, log))) = receiver.recv() {
                unsafe {
                    println!("setting log {}", log.as_str());
                    logger.log(level, log.as_str());
                }
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
    }
}
