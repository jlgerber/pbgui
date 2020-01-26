use super::*;

#[derive(Debug, PartialEq)]
pub enum MainWin {
    GetVpins,
    GetWithsForVpin,
    GetTransactionChanges,
    GetHistoryRevisions,
    SaveVpinChanges,
    ChooseDistribution,
}

impl ToEvent for MainWin {
    fn to_event(self) -> Event {
        Event::MainWin(self)
    }
}
//
impl ToQString for MainWin {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &MainWin::GetVpins => QString::from_std_str("MainWin::GetVpins"),
            &MainWin::GetWithsForVpin => QString::from_std_str("MainWin::GetWithsForVpin"),
            &MainWin::GetTransactionChanges => {
                QString::from_std_str("MainWin::GetTransactionChanges")
            }
            &MainWin::GetHistoryRevisions => QString::from_std_str("MainWin::GetHistoryRevisions"),
            &MainWin::SaveVpinChanges => QString::from_std_str("MainWin::SaveVpinChanges"),
            &MainWin::ChooseDistribution => QString::from_std_str("MainWin::ChooseDistribution"),
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
            "MainWin::SaveVpinChanges" => MainWin::SaveVpinChanges,
            "MainWin::ChooseDistribution" => MainWin::ChooseDistribution,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
