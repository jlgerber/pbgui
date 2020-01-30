use super::*;
use crate::messaging::event::ui_logger::UiLogger;
use crate::messaging::incoming::iui_logger::IUiLogger;
use crate::messaging::outgoing::oui_logger::OUiLogger;

/// perform a submatch against the OUiLogger msg
pub fn match_ui_logger<'a>(
    msg: OUiLogger<'a>,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg<'a>>,
) {
    match msg {
        OUiLogger::SendLog(record) => {
            // construct
            sender
                .send(IUiLogger::Log(record).to_imsg())
                .expect("unable to send logs");
            conductor.signal(UiLogger::SendLog.to_event());
        }
    }
}
