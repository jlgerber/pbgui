//! Event is used to formalize the qt signal that triggers an
//! update application side.
//! The messaging to the application from the db is split between the Event and the IMsg.
//! The Event signals that a given state has changed.
//! THe IMsg provides the details of the state change.
use qt_core::QString;
use qt_thread_conductor::traits::*;
use qt_widgets::cpp_core::{CppBox, Ref};

pub mod vpin_dialog;
pub use vpin_dialog::VpinDialog;

pub mod packages_tree;
pub use packages_tree::PackagesTree;

pub mod package_withs;
pub use package_withs::PackageWiths;

pub mod main_toolbar;
pub use main_toolbar::MainToolbar;
/// ergonomics related trait. Convert a nested enum to an event
pub trait ToEvent {
    fn to_event(self) -> Event;
}

#[derive(Debug, PartialEq)]
pub enum Event {
    VpinDialog(VpinDialog),
    PackagesTree(PackagesTree),
    PackageWiths(PackageWiths),
    MainToolbar(MainToolbar),
    Error,
}

impl ToQString for Event {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &Event::VpinDialog(vpin_dialog) => vpin_dialog.to_qstring(),
            &Event::PackagesTree(packages_tree) => packages_tree.to_qstring(),
            &Event::PackageWiths(package_withs) => package_withs.to_qstring(),
            &Event::MainToolbar(main_toolbar) => main_toolbar.to_qstring(),
            &Event::Error => QString::from_std_str("Error"),
        }
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        let test_str = qs.to_std_string();
        match test_str.as_str() {
            // delegate the work to the appropriate module
            test_str if test_str.starts_with("VpinDialog::") => {
                Event::VpinDialog(VpinDialog::from_qstring(qs))
            }
            test_str if test_str.starts_with("PackagesTree::") => {
                Event::PackagesTree(PackagesTree::from_qstring(qs))
            }
            test_str if test_str.starts_with("PackageWiths::") => {
                Event::PackageWiths(PackageWiths::from_qstring(qs))
            }
            test_str if test_str.starts_with("MainToolbar::") => {
                Event::MainToolbar(MainToolbar::from_qstring(qs))
            }
            "Error" => Event::Error,
            _ => panic!("Unable to convert to Event"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustqt_utils::qs;

    #[test]
    fn can_convert_from_event_to_qstring() {
        let event = Event::VpinDialog(VpinDialog::UpdateRoles);
        assert_eq!(
            event.to_qstring().to_std_string().as_str(),
            "VpinDialog::UpdateRoles"
        );
    }
    #[test]
    fn can_convert_from_qstring() {
        let qstr = qs("VpinDialog::UpdateRoles");
        let qstr_ref = unsafe { qstr.as_ref() };
        let event = Event::from_qstring(qstr_ref);
        assert_eq!(event, Event::VpinDialog(VpinDialog::UpdateRoles));
    }
}
