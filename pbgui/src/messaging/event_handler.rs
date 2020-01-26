use crate::main_window::InnerMainWindow;
use crate::messaging::{prelude::*, Event, IMsg, IVpinDialog, VpinDialog};
use crossbeam_channel::Receiver;
use log;
use pbgui_vpin::vpin_dialog;
use qt_core::{QString, SlotOfQString};
use qt_widgets::cpp_core::Ref;
use std::rc::Rc;

pub mod vpin_dialog_eh;
use vpin_dialog_eh::match_vpin_dialog;

pub mod main_toolbar_eh;
use main_toolbar_eh::match_main_toolbar;

pub mod package_withs_eh;
use package_withs_eh::match_package_withs;

pub mod packages_tree_eh;
use packages_tree_eh::match_packages_tree;

pub mod main_win_eh;
use main_win_eh::match_main_win;
/// Generate a new event handler, which is of type `SlotOfQString`.
/// The event handler is responsible for handling Signals of type Event
///
/// # Arguments
/// * `dialog` - reference counted pointer to VpinDialog instance
/// * `main` - reference counted pointer to the InnerMainwindow instance
/// * `receiver` - The Receiver of messages from the non-ui thread
///
/// # Returns
/// * Slot which processes messages from the non-ui thread and updates the ui in response
pub fn new_event_handler<'a>(
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
    main: Rc<InnerMainWindow<'a>>,
    receiver: Receiver<IMsg>,
) -> SlotOfQString<'a> {
    SlotOfQString::new(move |name: Ref<QString>| unsafe {
        let tree = main.tree();
        let withs = main.package_withs_list();
        let main_toolbar = main.main_toolbar();

        match Event::from_qstring(name) {
            Event::VpinDialog(vpin_dialog_event) => {
                match_vpin_dialog(vpin_dialog_event, dialog.clone(), &receiver)
            }
            Event::PackagesTree(packages_tree_event) => {
                match_packages_tree(packages_tree_event, tree, &receiver)
            }
            Event::PackageWiths(package_withs_event) => {
                match_package_withs(package_withs_event, withs, &receiver)
            }
            Event::MainToolbar(main_toolbar_event) => {
                match_main_toolbar(main_toolbar_event, main_toolbar, &receiver)
            }
            Event::MainWin(main_win_event) => {
                match_main_win(main_win_event, main.clone(), &receiver)
            }
            Event::Error => {
                if let Ok(IMsg::Error(error)) = receiver.recv() {
                    log::error!("{}", error);
                } else {
                    log::error!("unable to transmit error");
                }
            }
        }
    })
}
