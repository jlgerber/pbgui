//! Provides the UiLogger enum, which implements the ToEvent, ToQString, and FromQString traits

use super::*;

#[derive(Debug, PartialEq)]
pub enum UiLogger {
    SendLog,
}

impl ToEvent for UiLogger {
    fn to_event(self) -> Event {
        Event::UiLogger(self)
    }
}
//
impl ToQString for UiLogger {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &UiLogger::SendLog => QString::from_std_str("UiLogger::SendLog"),
        }
    }
}

impl FromQString for UiLogger {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "UiLogger::SendLog" => UiLogger::SendLog,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
