use super::*;
use log::Record;

#[derive(Debug)]
pub enum OUiLogger<'a> {
    SendLog(Record<'a>),
}

impl<'a> ToOMsg<'a> for OUiLogger<'a> {
    fn to_omsg(self) -> OMsg<'a> {
        OMsg::UiLogger(self)
    }
}
