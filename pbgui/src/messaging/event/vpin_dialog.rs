use super::*;

#[derive(Debug, PartialEq)]
pub enum VpinDialog {
    UpdateRoles,
    UpdateSites,
    UpdateLevels,
}

impl ToEvent for VpinDialog {
    fn to_event(self) -> Event {
        Event::VpinDialog(self)
    }
}

impl ToQString for VpinDialog {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &VpinDialog::UpdateRoles => QString::from_std_str("VpinDialog::UpdateRoles"),
            &VpinDialog::UpdateSites => QString::from_std_str("VpinDialog::UpdateSites"),
            &VpinDialog::UpdateLevels => QString::from_std_str("VpinDialog::UpdateLevels"),
        }
    }
}

impl FromQString for VpinDialog {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "VpinDialog::UpdateRoles" => VpinDialog::UpdateRoles,
            "VpinDialog::UpdateSites" => VpinDialog::UpdateSites,
            "VpinDialog::UpdateLevels" => VpinDialog::UpdateLevels,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
