use super::*;

#[derive(Debug, PartialEq)]
pub enum MainWin {
    GetVpins,
    GetWithsForVpin,
    GetTransactionChanges,
    GetHistoryRevisions,
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
            &MainWin::GetWithsForVpin => QString::from_std_str("MainWin::GetWithsForVpin"),
            &MainWin::GetTransactionChanges => {
                QString::from_std_str("MainWin::GetTransactionChanges")
            }
            &MainWin::GetHistoryRevisions => QString::from_std_str("MainWin::GetHistoryRevisions"),
        }
    }
}

impl FromQString for MainWin {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "MainWin::GetVpins" => MainWin::GetVpins,
            "MainWin::GetWithsForVpin" => MainWin::GetWithsForVpin,
            "MainWin::GetTransactionChanges" => MainWin::GetTransactionChanges,
            "MainWin::GetHistoryRevisions" => MainWin::GetHistoryRevisions,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
