//! Provides the VpinDialog enum, which implements the ToEvent, ToQString, and FromQString traits
use super::*;

#[derive(Debug, PartialEq)]
pub enum VpinDialog {
    UpdateRoles,
    UpdateSites,
    UpdateLevels,
    SetShow,
    SetVpin,
}

impl ToEvent for VpinDialog {
    fn to_event(self) -> Event {
        Event::VpinDialog(self)
    }
}

impl ToQString for VpinDialog {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            VpinDialog::UpdateRoles => QString::from_std_str("VpinDialog::UpdateRoles"),
            VpinDialog::UpdateSites => QString::from_std_str("VpinDialog::UpdateSites"),
            VpinDialog::UpdateLevels => QString::from_std_str("VpinDialog::UpdateLevels"),
            VpinDialog::SetShow => QString::from_std_str("VpinDialog::SetShow"),
            VpinDialog::SetVpin => QString::from_std_str("VpinDialog::SetVpin"),
        }
    }
}

impl FromQString for VpinDialog {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "VpinDialog::UpdateRoles" => VpinDialog::UpdateRoles,
            "VpinDialog::UpdateSites" => VpinDialog::UpdateSites,
            "VpinDialog::UpdateLevels" => VpinDialog::UpdateLevels,
            "VpinDialog::SetShow" => VpinDialog::SetShow,
            "VpinDialog::SetVpin" => VpinDialog::SetVpin,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
