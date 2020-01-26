use super::*;

#[derive(Debug, PartialEq)]
pub enum MainWin {
    GetVpins,
}

impl ToEvent for MainWin {
    fn to_event(self) -> Event {
        Event::MainWin(self)
    }
}

impl ToQString for MainWin {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &MainWin::GetVpins => QString::from_std_str("MainWin::GetVpins"),
        }
    }
}

impl FromQString for MainWin {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "MainWin::GetVpins" => MainWin::GetVpins,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
