use crate::{prelude::*, Event, IMsg, IVpinDialog, VpinDialog};
use crossbeam_channel::Receiver;
use log;
use pbgui_toolbar::toolbar::MainToolbar;
use pbgui_tree::tree;
use pbgui_vpin::vpin_dialog;
use pbgui_withs::WithsList;
use qt_core::{QString, SlotOfQString};
use qt_widgets::cpp_core::Ref;
use std::cell::RefCell;
use std::rc::Rc;

pub mod vpin_dialog_eh;
use vpin_dialog_eh::match_vpin_dialog;
pub mod main_toolbar_eh;
pub mod package_withs_eh;
pub mod packages_tree_eh;
use main_toolbar_eh::match_main_toolbar;
use package_withs_eh::match_package_withs;
use packages_tree_eh::match_packages_tree;
/// Generate a new event handler, which is of type `SlotOfQString`.
/// The event handler is responsible for handling Signals of type Event
///
/// # Arguments
/// * `dialog` - Rc wrapped VpinDialog
/// * `receiver` - The Receiver of messages from the non-ui thread
///
/// # Returns
/// * Slot which processes messages from the non-ui thread and updates the ui in response
pub fn new_event_handler<'a>(
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
    tree: Rc<RefCell<tree::DistributionTreeView<'a>>>,
    withs: Rc<RefCell<WithsList<'a>>>,
    main_toolbar: Rc<MainToolbar>,
    receiver: Receiver<IMsg>,
) -> SlotOfQString<'a> {
    SlotOfQString::new(move |name: Ref<QString>| match Event::from_qstring(name) {
        //
        Event::VpinDialog(vpin_dialog_event) => {
            match_vpin_dialog(vpin_dialog_event, dialog.clone(), &receiver)
        }
        Event::PackagesTree(packages_tree_event) => {
            match_packages_tree(packages_tree_event, tree.clone(), &receiver)
        }
        Event::PackageWiths(package_withs_event) => {
            match_package_withs(package_withs_event, withs.clone(), &receiver)
        }
        Event::MainToolbar(main_toolbar_event) => {
            match_main_toolbar(main_toolbar_event, main_toolbar.clone(), &receiver)
        }
        //
        Event::Error => {
            if let Ok(IMsg::Error(error)) = receiver.recv() {
                log::error!("{}", error);
            } else {
                log::error!("unable to transmit error");
            }
        }
    })
}
