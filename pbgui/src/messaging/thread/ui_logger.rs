use super::*;
use crate::messaging::event::ui_logger::UiLogger;
use crate::messaging::incoming::iui_logger::IUiLogger;
use crate::messaging::outgoing::oui_logger::OUiLogger;

/// perform a submatch against the OUiLogger msg
pub fn match_ui_logger(
    msg: OUiLogger,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg>,
) {
    match msg {
        OUiLogger::SendLog {
            level,
            target,
            file,
            line,
            msg,
        } => {
            // construct
            sender
                .send(
                    IUiLogger::Log {
                        level,
                        target,
                        file,
                        line,
                        msg,
                    }
                    .to_imsg(),
                )
                .expect("unable to send logs");
            conductor.signal(UiLogger::SendLog.to_event());
        }
    }
}
