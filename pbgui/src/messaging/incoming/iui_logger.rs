use super::*;
use log::Record;

pub enum IUiLogger<'a> {
    Log(Record<'a>),
}

impl<'a> ToIMsg<'a> for IUiLogger<'a> {
    fn to_imsg(self) -> IMsg<'a> {
        IMsg::UiLogger(self)
    }
}
